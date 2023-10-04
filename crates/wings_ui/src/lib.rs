pub mod common_tags;
pub mod enums;
pub mod events;
pub mod plugin;
pub mod queries;
pub(crate) mod systems;
pub mod widgets;

pub mod prelude {
    pub use crate::widgets::{
        UiButton, UiCenter, UiColumn, UiConstrainedHeight, UiConstrainedWidth, UiContainer,
        UiHorizontalDivider, UiRow, UiScaffold, UiVerticalDivider, UiWidgetBundle,
    };

    pub use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};

    pub use crate::events::{
        UiPointerClick, UiPointerDoubleClick, UiPointerEnter, UiPointerExit,
        UiPointerMove, UiPointerOver, UiPointerPress, UiPointerRelease,
    };

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::queries::color::UiColorQuery;
    pub use crate::queries::size::UiSizeQuery;
    pub use crate::queries::visibility::{UiLayoutVisibilityQuery, UiVisibilityQuery};

    pub use crate::common_tags::UiTagCollapsible;

    pub use crate::widgets::alignment::UiCenterBundle;
    pub use crate::widgets::button::{UiButtonBundle, UiButtonProps};
    pub use crate::widgets::constrained::{UiConstrainedHeightBundle, UiConstrainedWidthBundle};
    pub use crate::widgets::container::{UiContainerBundle, UiContainerProps};
    pub use crate::widgets::direction::{UiColumnBundle, UiColumnProps, UiRowBundle, UiRowProps};
    pub use crate::widgets::divider::{
        UiHorizontalDividerBundle, UiHorizontalDividerProps,
        UiVerticalDividerBundle, UiVerticalDividerProps,
    };
    pub use crate::widgets::scaffold::{UiScaffoldBundle, UiScaffoldProps};
    pub use crate::widgets::sized_box::{UiSizedBoxBundle, UiSizedBoxProps};
    pub use crate::widgets::visibility::{
        UiLayoutVisibility, UiLayoutVisibilityBundle, UiVisibility, UiVisibilityBundle,
    };
}
