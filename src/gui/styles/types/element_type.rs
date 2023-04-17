/// Used to specify the kind of `iced` element, to be able to choose the appropriate style for it
#[derive(Clone, Copy)]
pub enum ElementType {
    Default,
    NavColumn,
    NavHeader,
    NavButton,
    NavButtonSelected,
    ItemListColumn,
    ItemListEntry,
    SelectedItem,
    DetailsColumn,
    PrimaryText,
    SecondaryText,
    PrimaryAccent,
    SecondaryAccent,
    Button,
    ButtonText,
    Border,
}
