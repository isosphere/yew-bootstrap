use yew::prelude::*;
use std::rc::Rc;
use super::SOption;

#[cfg(doc)]
use super::SearchableSelect;

/// Type of the function inside [FilterFn] structure.
///
/// Arguments are:
///
/// - value (AttrValue): value to search for the filtering
/// - option (&SOption): Option to filter
///
/// The function returns `true` if the option needs to be kept
type FilterFnInner = dyn Fn(AttrValue, &SOption) -> bool;

/// # Filtering structure for [SearchableSelect]
///
/// Structure containing a function for filtering. It can be used to
/// override the function inside [SearchableSelect].
///
/// These functions are expected to be common and are provided:
///
/// - [filter_icase]: filter case-insensitive on the options. This is the
///   the default filtering function
/// - [filter_case]: filter case-sensivitive. You can use it inside
///   [SearchableSelect] by simply calling it:
///
///   ```rust
///   use yew::prelude::*;
///   use yew_bootstrap::component::{SearchableSelect, SOption, filter_case};
///
///   fn test() -> Html {
///       html! {
///           <SearchableSelect
///               options={ vec![]}
///               title=""
///               placeholder="Select an option..."
///               onselectchange={ Callback::default() }
///               filter={ filter_case() }
///           />
///       }
///   }
///   ```
///
/// You can also define your own function, for example:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::FilterFn;
///
/// let my_filter = FilterFn::from(|value: AttrValue, option: &SOption|
///   option.title.starts_with(value.as_str())
/// );
/// ```
///
/// And use `filter = my_filter` in [SearchableSelect] component.
///
#[derive(Clone)]
pub struct FilterFn (
    /// Internal function, inside [Rc] for cheap Clone implementation.
    pub Rc<FilterFnInner>
);

impl PartialEq for FilterFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Implement `From` to help creation of `FilterFn`. Example:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::FilterFn;
///
/// let my_filter = FilterFn::from(|value: AttrValue, option: &SOption|
///   option.title.starts_with(value.as_str())
/// );
/// ```
impl<F> From<F> for FilterFn
where
    F: Fn(AttrValue, &SOption) -> bool + 'static,
{
    fn from(f: F) -> Self {
        FilterFn(Rc::new(f))
    }
}

/// # Filter function (Case insensitive)
///
/// Returns true if option contains provided value (case insensitive). This is the
/// default filter function inside [SearchableSelect] component.
pub fn filter_icase() -> FilterFn {
    FilterFn::from(|value: AttrValue, option: &SOption| {
        option.title.to_lowercase().contains(&value.to_lowercase())
    })
}

/// # Filter function (Case sensitive)
///
/// Returns true if option contains provided value (case sensitive)
///
/// To use it inside [SearchableSelect], set `filter = filter_case()`.
pub fn filter_case() -> FilterFn {
    FilterFn::from(|value: AttrValue, option: &SOption| {
        option.title.as_ref().contains(value.as_ref())
    })
}

/// Filter by group
///
/// Filter options by considering all headers are creating a new group, using
/// the filter function. If a group is empty, it is removed from the list. Return
/// a GroupFilter structure to get index of previous and next items.
///
/// Return a vector of tuple containing:
/// - original index in the list, as integer
/// - the reference to the option
pub (crate) fn filter_by_group(options: &[SOption], search: AttrValue, filter: FilterFn) -> Vec<(usize, &SOption)> {
    let mut result = Vec::new();
    let mut current_group = Vec::new();
    let mut current_header = None;

    for (idx, option) in options.iter().enumerate() {
        if option.header {
            // Process previous group if exists
            if !current_group.is_empty() {
                if let Some(header) = current_header {
                    result.push(header);
                }
                result.extend(current_group);
            }
            current_group = Vec::new();
            current_header = Some((idx, option));
        } else if filter.0(search.clone(), option) {
            current_group.push((idx, option));
        }
    }

    // Handle last group
    if !current_group.is_empty() {
        if let Some(header) = current_header {
            result.push(header);
        }
        result.extend(current_group);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_filter_icase() {
        let items = [
            SOption{ title: AttrValue::from("Apple"), ..SOption::default()},
            SOption{ title: AttrValue::from("Banana"), ..SOption::default()},
            SOption{ title: AttrValue::from("Cherry"), ..SOption::default()},
            SOption{ title: AttrValue::from("Date"), ..SOption::default()},
            SOption{ title: AttrValue::from("Elderberry"), ..SOption::default()},
            SOption{ title: AttrValue::from("Fig"), ..SOption::default()},
            SOption{ title: AttrValue::from("Grapes"), ..SOption::default()},
        ];

        let filter_fn = filter_icase().0;

        let value = AttrValue::from("Er");
        let result: Vec<&SOption> = items.iter().filter(
            |option| filter_fn(value.clone(), option)
        ).collect();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, AttrValue::from("Cherry"));
        assert_eq!(result[1].title, AttrValue::from("Elderberry"));
    }

    #[test]
    pub fn test_filter_case() {
        let items = [
            SOption{ title: AttrValue::from("Apple"), ..SOption::default()},
            SOption{ title: AttrValue::from("Banana"), ..SOption::default()},
            SOption{ title: AttrValue::from("Cherry"), ..SOption::default()},
            SOption{ title: AttrValue::from("Date"), ..SOption::default()},
            SOption{ title: AttrValue::from("Elderberry"), ..SOption::default()},
            SOption{ title: AttrValue::from("Fig"), ..SOption::default()},
            SOption{ title: AttrValue::from("Grapes"), ..SOption::default()},
        ];

        let filter_fn = filter_case().0;

        let value = AttrValue::from("Er");
        let result: Vec<&SOption> = items.iter().filter(
            |option| filter_fn(value.clone(), option)
        ).collect();
        assert_eq!(result.len(), 0);

        let value = AttrValue::from("Ba");
        let result: Vec<&SOption> = items.iter().filter(
            |option| filter_fn(value.clone(), option)
        ).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, AttrValue::from("Banana"));
    }

    #[test]
    fn test_from() {
        let filter = FilterFn::from(|value: AttrValue, option: &SOption|
            option.title.starts_with(value.as_str())
        );

        let option = SOption{ title: "Test".into(), ..SOption::default()};

        assert_eq!(filter.0("Te".into(), &option), true);
        assert_eq!(filter.0("est".into(), &option), false);
    }

    #[test]
    fn test_filter_by_group() {
        let filter_fn = filter_icase();

        let get_titles = |options: Vec<(usize, &SOption)>| {
            options.iter().map(|(i, option)| (*i, option.title.clone())).collect::<Vec<(usize, AttrValue)>>()
        };

        let options = vec![
            SOption { title: "Item A".into(), header: false, ..SOption::default() },
            SOption { title: "Item B".into(), header: false, ..SOption::default() },
            SOption { title: "Item C".into(), header: false, ..SOption::default() },
        ];

        let filtered = filter_by_group(&options, "item".into(), filter_fn.clone());
        assert_eq!(
            get_titles(filtered),
            vec![
                (0, "Item A".into()),
                (1, "Item B".into()),
                (2, "Item C".into()),
            ]
        );

        let options = vec![
            SOption { title: "Item A0".into(), header: false, ..SOption::default() },
            SOption { title: "Group 1".into(), header: true, ..SOption::default() },
            SOption { title: "Item B0".into(), header: false, ..SOption::default() },
            SOption { title: "Item B1".into(), header: false, ..SOption::default() },
            SOption { title: "Group 2".into(), header: true, ..SOption::default() },
            SOption { title: "Item C0".into(), header: false, ..SOption::default() },
        ];

        let filtered = filter_by_group(&options, "0".into(), filter_fn.clone());
        assert_eq!(
            get_titles(filtered),
            vec![
                (0, "Item A0".into()),
                (1, "Group 1".into()),
                (2, "Item B0".into()),
                (4, "Group 2".into()),
                (5, "Item C0".into()),
            ]
        );

        let filtered = filter_by_group(&options, "1".into(), filter_fn.clone());
        assert_eq!(
            get_titles(filtered),
            vec![
                (1, "Group 1".into()),
                (3, "Item B1".into()),
            ]
        );

        let filtered = filter_by_group(&options, "c".into(), filter_fn.clone());
        assert_eq!(
            get_titles(filtered),
            vec![
                (4, "Group 2".into()),
                (5, "Item C0".into()),
            ]
        );
    }

}