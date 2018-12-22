#include <QApplication>
#include <QColor>
#include <QContextMenuEvent>
#include <QCoreApplication>
#include <QFont>
#include <QGradient>
#include <QIcon>
#include <QImage>
#include <QKeySequence>
#include <QLayout>
#include <QLineEdit>
#include <QLineF>
#include <QListWidgetItem>
#include <QPaintDevice>
#include <QPaintEngine>
#include <QPainter>
#include <QPalette>
#include <QPixelFormat>
#include <QRegion>
#include <QSizePolicy>
#include <QStyle>
#include <QSurface>
#include <QSurfaceFormat>
#include <QTabletEvent>
#include <QToolButton>
#include <QTransform>
#include <QWheelEvent>
#include <QWidget>
#include <QWindow>
#include <Qt>
#include <map>

struct KeyVal {
    unsigned int val, key;
};

std::map<int, int> s_action_position_lookup;
std::map<int, int> s_alignment_flag_lookup;
std::map<int, int> s_alpha_position_lookup;
std::map<int, int> s_alpha_premultiplied_lookup;
std::map<int, int> s_alpha_usage_lookup;
std::map<int, int> s_ancestor_mode_lookup;
std::map<int, int> s_anchor_point_lookup;
std::map<int, int> s_application_attribute_lookup;
std::map<int, int> s_application_state_lookup;
std::map<int, int> s_arrow_type_lookup;
std::map<int, int> s_aspect_ratio_mode_lookup;
std::map<int, int> s_axis_lookup;
std::map<int, int> s_bg_mode_lookup;
std::map<int, int> s_brush_style_lookup;
std::map<int, int> s_byte_order_lookup;
std::map<int, int> s_capitalization_lookup;
std::map<int, int> s_case_sensitivity_lookup;
std::map<int, int> s_check_state_lookup;
std::map<int, int> s_checksum_type_lookup;
std::map<int, int> s_clip_operation_lookup;
std::map<int, int> s_color_group_lookup;
std::map<int, int> s_color_model_lookup;
std::map<int, int> s_color_role_lookup;
std::map<int, int> s_color_space_lookup;
std::map<int, int> s_color_spec_lookup;
std::map<int, int> s_complex_control_lookup;
std::map<int, int> s_composition_mode_lookup;
std::map<int, int> s_connection_type_lookup;
std::map<int, int> s_contents_type_lookup;
std::map<int, int> s_context_menu_policy_lookup;
std::map<int, int> s_control_element_lookup;
std::map<int, int> s_control_type_lookup;
std::map<int, int> s_coordinate_mode_lookup;
std::map<int, int> s_coordinate_system_lookup;
std::map<int, int> s_core_application_fix_me_enums_lookup;
std::map<int, int> s_corner_lookup;
std::map<int, int> s_cursor_move_style_lookup;
std::map<int, int> s_cursor_shape_lookup;
std::map<int, int> s_date_format_lookup;
std::map<int, int> s_day_of_week_lookup;
std::map<int, int> s_dirty_flag_lookup;
std::map<int, int> s_dock_widget_area_lookup;
std::map<int, int> s_dock_widget_area_sizes_lookup;
std::map<int, int> s_drop_action_lookup;
std::map<int, int> s_echo_mode_lookup;
std::map<int, int> s_edge_lookup;
std::map<int, int> s_enter_key_type_lookup;
std::map<int, int> s_event_priority_lookup;
std::map<int, int> s_fill_rule_lookup;
std::map<int, int> s_find_child_option_lookup;
std::map<int, int> s_focus_policy_lookup;
std::map<int, int> s_focus_reason_lookup;
std::map<int, int> s_format_lookup;
std::map<int, int> s_format_option_lookup;
std::map<int, int> s_gesture_flag_lookup;
std::map<int, int> s_gesture_state_lookup;
std::map<int, int> s_gesture_type_lookup;
std::map<int, int> s_global_color_lookup;
std::map<int, int> s_hinting_preference_lookup;
std::map<int, int> s_hit_test_accuracy_lookup;
std::map<int, int> s_image_conversion_flag_lookup;
std::map<int, int> s_initialization_lookup;
std::map<int, int> s_input_method_hint_lookup;
std::map<int, int> s_input_method_query_lookup;
std::map<int, int> s_interpolation_mode_lookup;
std::map<int, int> s_intersect_type_lookup;
std::map<int, int> s_invert_mode_lookup;
std::map<int, int> s_item_data_role_lookup;
std::map<int, int> s_item_flag_lookup;
std::map<int, int> s_item_selection_mode_lookup;
std::map<int, int> s_item_selection_operation_lookup;
std::map<int, int> s_item_type_lookup;
std::map<int, int> s_key_lookup;
std::map<int, int> s_keyboard_modifier_lookup;
std::map<int, int> s_layout_direction_lookup;
std::map<int, int> s_mask_mode_lookup;
std::map<int, int> s_match_flag_lookup;
std::map<int, int> s_mode_lookup;
std::map<int, int> s_modifier_lookup;
std::map<int, int> s_mouse_button_lookup;
std::map<int, int> s_mouse_event_flag_lookup;
std::map<int, int> s_mouse_event_source_lookup;
std::map<int, int> s_name_format_lookup;
std::map<int, int> s_native_gesture_type_lookup;
std::map<int, int> s_navigation_mode_lookup;
std::map<int, int> s_open_gl_context_profile_lookup;
std::map<int, int> s_orientation_lookup;
std::map<int, int> s_paint_device_metric_lookup;
std::map<int, int> s_paint_engine_feature_lookup;
std::map<int, int> s_pen_cap_style_lookup;
std::map<int, int> s_pen_join_style_lookup;
std::map<int, int> s_pen_style_lookup;
std::map<int, int> s_pixel_metric_lookup;
std::map<int, int> s_pixmap_fragment_hint_lookup;
std::map<int, int> s_pointer_type_lookup;
std::map<int, int> s_policy_lookup;
std::map<int, int> s_policy_flag_lookup;
std::map<int, int> s_polygon_draw_mode_lookup;
std::map<int, int> s_primitive_element_lookup;
std::map<int, int> s_reason_lookup;
std::map<int, int> s_region_type_lookup;
std::map<int, int> s_render_flag_lookup;
std::map<int, int> s_render_hint_lookup;
std::map<int, int> s_renderable_type_lookup;
std::map<int, int> s_request_software_input_panel_lookup;
std::map<int, int> s_screen_orientation_lookup;
std::map<int, int> s_scroll_bar_policy_lookup;
std::map<int, int> s_scroll_phase_lookup;
std::map<int, int> s_sequence_format_lookup;
std::map<int, int> s_sequence_match_lookup;
std::map<int, int> s_shortcut_context_lookup;
std::map<int, int> s_size_constraint_lookup;
std::map<int, int> s_size_hint_lookup;
std::map<int, int> s_size_mode_lookup;
std::map<int, int> s_sort_order_lookup;
std::map<int, int> s_spacing_type_lookup;
std::map<int, int> s_spec_lookup;
std::map<int, int> s_spread_lookup;
std::map<int, int> s_standard_key_lookup;
std::map<int, int> s_standard_pixmap_lookup;
std::map<int, int> s_state_lookup;
std::map<int, int> s_state_flag_lookup;
std::map<int, int> s_stretch_lookup;
std::map<int, int> s_style_lookup;
std::map<int, int> s_style_hint_lookup;
std::map<int, int> s_style_strategy_lookup;
std::map<int, int> s_sub_control_lookup;
std::map<int, int> s_sub_element_lookup;
std::map<int, int> s_surface_class_lookup;
std::map<int, int> s_surface_type_lookup;
std::map<int, int> s_swap_behavior_lookup;
std::map<int, int> s_tab_focus_behavior_lookup;
std::map<int, int> s_tablet_device_lookup;
std::map<int, int> s_text_elide_mode_lookup;
std::map<int, int> s_text_flag_lookup;
std::map<int, int> s_text_format_lookup;
std::map<int, int> s_text_interaction_flag_lookup;
std::map<int, int> s_tile_rule_lookup;
std::map<int, int> s_time_spec_lookup;
std::map<int, int> s_timer_type_lookup;
std::map<int, int> s_tool_bar_area_lookup;
std::map<int, int> s_tool_bar_area_sizes_lookup;
std::map<int, int> s_tool_button_popup_mode_lookup;
std::map<int, int> s_tool_button_style_lookup;
std::map<int, int> s_touch_point_state_lookup;
std::map<int, int> s_transformation_mode_lookup;
std::map<int, int> s_transformation_type_lookup;
std::map<int, int> s_type_lookup;
std::map<int, int> s_type_interpretation_lookup;
std::map<int, int> s_ui_effect_lookup;
std::map<int, int> s_visibility_lookup;
std::map<int, int> s_weight_lookup;
std::map<int, int> s_wheel_event_fix_me_enums_lookup;
std::map<int, int> s_white_space_mode_lookup;
std::map<int, int> s_widget_attribute_lookup;
std::map<int, int> s_window_frame_section_lookup;
std::map<int, int> s_window_modality_lookup;
std::map<int, int> s_window_state_lookup;
std::map<int, int> s_window_type_lookup;
std::map<int, int> s_yuv_layout_lookup;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern void create_enum_mappings() {

    static KeyVal action_position_vals[] = {
        {(unsigned int)QLineEdit::LeadingPosition, 0},
        {(unsigned int)QLineEdit::TrailingPosition, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_action_position_lookup[action_position_vals[i].key] =
            (unsigned int)action_position_vals[i].val;
    }

    static KeyVal alignment_flag_vals[] = {
        {(unsigned int)Qt::AlignLeft, 1},
        {(unsigned int)Qt::AlignLeading, 1},
        {(unsigned int)Qt::AlignRight, 2},
        {(unsigned int)Qt::AlignTrailing, 2},
        {(unsigned int)Qt::AlignHCenter, 4},
        {(unsigned int)Qt::AlignJustify, 8},
        {(unsigned int)Qt::AlignAbsolute, 16},
        {(unsigned int)Qt::AlignHorizontal_Mask, 31},
        {(unsigned int)Qt::AlignTop, 32},
        {(unsigned int)Qt::AlignBottom, 64},
        {(unsigned int)Qt::AlignVCenter, 128},
        {(unsigned int)Qt::AlignBaseline, 256},
        {(unsigned int)Qt::AlignVertical_Mask, 480},
        {(unsigned int)Qt::AlignCenter, 132},
    };

    for (int i = 0; i < 14; ++i) {
        s_alignment_flag_lookup[alignment_flag_vals[i].key] =
            (unsigned int)alignment_flag_vals[i].val;
    }

    static KeyVal alpha_position_vals[] = {
        {(unsigned int)QPixelFormat::AtBeginning, 0},
        {(unsigned int)QPixelFormat::AtEnd, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_position_lookup[alpha_position_vals[i].key] =
            (unsigned int)alpha_position_vals[i].val;
    }

    static KeyVal alpha_premultiplied_vals[] = {
        {(unsigned int)QPixelFormat::NotPremultiplied, 0},
        {(unsigned int)QPixelFormat::Premultiplied, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_premultiplied_lookup[alpha_premultiplied_vals[i].key] =
            (unsigned int)alpha_premultiplied_vals[i].val;
    }

    static KeyVal alpha_usage_vals[] = {
        {(unsigned int)QPixelFormat::UsesAlpha, 0},
        {(unsigned int)QPixelFormat::IgnoresAlpha, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_usage_lookup[alpha_usage_vals[i].key] =
            (unsigned int)alpha_usage_vals[i].val;
    }

    static KeyVal ancestor_mode_vals[] = {
        {(unsigned int)QWindow::ExcludeTransients, 0},
        {(unsigned int)QWindow::IncludeTransients, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_ancestor_mode_lookup[ancestor_mode_vals[i].key] =
            (unsigned int)ancestor_mode_vals[i].val;
    }

    static KeyVal anchor_point_vals[] = {
        {(unsigned int)Qt::AnchorLeft, 0},
        {(unsigned int)Qt::AnchorHorizontalCenter, 1},
        {(unsigned int)Qt::AnchorRight, 2},
        {(unsigned int)Qt::AnchorTop, 3},
        {(unsigned int)Qt::AnchorVerticalCenter, 4},
        {(unsigned int)Qt::AnchorBottom, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_anchor_point_lookup[anchor_point_vals[i].key] =
            (unsigned int)anchor_point_vals[i].val;
    }

    static KeyVal application_attribute_vals[] = {
        {(unsigned int)Qt::AA_ImmediateWidgetCreation, 0},
        {(unsigned int)Qt::AA_MSWindowsUseDirect3DByDefault, 1},
        {(unsigned int)Qt::AA_DontShowIconsInMenus, 2},
        {(unsigned int)Qt::AA_NativeWindows, 3},
        {(unsigned int)Qt::AA_DontCreateNativeWidgetSiblings, 4},
        {(unsigned int)Qt::AA_PluginApplication, 5},
        {(unsigned int)Qt::AA_DontUseNativeMenuBar, 6},
        {(unsigned int)Qt::AA_MacDontSwapCtrlAndMeta, 7},
        {(unsigned int)Qt::AA_Use96Dpi, 8},
        {(unsigned int)Qt::AA_X11InitThreads, 10},
        {(unsigned int)Qt::AA_SynthesizeTouchForUnhandledMouseEvents, 11},
        {(unsigned int)Qt::AA_SynthesizeMouseForUnhandledTouchEvents, 12},
        {(unsigned int)Qt::AA_UseHighDpiPixmaps, 13},
        {(unsigned int)Qt::AA_ForceRasterWidgets, 14},
        {(unsigned int)Qt::AA_UseDesktopOpenGL, 15},
        {(unsigned int)Qt::AA_UseOpenGLES, 16},
        {(unsigned int)Qt::AA_UseSoftwareOpenGL, 17},
        {(unsigned int)Qt::AA_ShareOpenGLContexts, 18},
        {(unsigned int)Qt::AA_SetPalette, 19},
        {(unsigned int)Qt::AA_DisableHighDpiScaling, 21},
        {(unsigned int)Qt::AA_UseStyleSheetPropagationInWidgetStyles, 22},
        {(unsigned int)Qt::AA_DontUseNativeDialogs, 23},
        {(unsigned int)Qt::AA_SynthesizeMouseForUnhandledTabletEvents, 24},
        {(unsigned int)Qt::AA_CompressHighFrequencyEvents, 25},
        {(unsigned int)Qt::AA_DontCheckOpenGLContextThreadAffinity, 26},
        {(unsigned int)Qt::AA_DisableShaderDiskCache, 27},
        {(unsigned int)Qt::AA_DontShowShortcutsInContextMenus, 28},
        {(unsigned int)Qt::AA_CompressTabletEvents, 29},
        {(unsigned int)Qt::AA_DisableWindowContextHelpButton, 30},
        {(unsigned int)Qt::AA_AttributeCount, 31},
    };

    for (int i = 0; i < 30; ++i) {
        s_application_attribute_lookup[application_attribute_vals[i].key] =
            (unsigned int)application_attribute_vals[i].val;
    }

    static KeyVal application_state_vals[] = {
        {(unsigned int)Qt::ApplicationSuspended, 0},
        {(unsigned int)Qt::ApplicationHidden, 1},
        {(unsigned int)Qt::ApplicationInactive, 2},
        {(unsigned int)Qt::ApplicationActive, 4},
    };

    for (int i = 0; i < 4; ++i) {
        s_application_state_lookup[application_state_vals[i].key] =
            (unsigned int)application_state_vals[i].val;
    }

    static KeyVal arrow_type_vals[] = {
        {(unsigned int)Qt::NoArrow, 0},    {(unsigned int)Qt::UpArrow, 1},
        {(unsigned int)Qt::DownArrow, 2},  {(unsigned int)Qt::LeftArrow, 3},
        {(unsigned int)Qt::RightArrow, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_arrow_type_lookup[arrow_type_vals[i].key] =
            (unsigned int)arrow_type_vals[i].val;
    }

    static KeyVal aspect_ratio_mode_vals[] = {
        {(unsigned int)Qt::IgnoreAspectRatio, 0},
        {(unsigned int)Qt::KeepAspectRatio, 1},
        {(unsigned int)Qt::KeepAspectRatioByExpanding, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_aspect_ratio_mode_lookup[aspect_ratio_mode_vals[i].key] =
            (unsigned int)aspect_ratio_mode_vals[i].val;
    }

    static KeyVal axis_vals[] = {
        {(unsigned int)Qt::XAxis, 0},
        {(unsigned int)Qt::YAxis, 1},
        {(unsigned int)Qt::ZAxis, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_axis_lookup[axis_vals[i].key] = (unsigned int)axis_vals[i].val;
    }

    static KeyVal bg_mode_vals[] = {
        {(unsigned int)Qt::TransparentMode, 0},
        {(unsigned int)Qt::OpaqueMode, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_bg_mode_lookup[bg_mode_vals[i].key] =
            (unsigned int)bg_mode_vals[i].val;
    }

    static KeyVal brush_style_vals[] = {
        {(unsigned int)Qt::NoBrush, 0},
        {(unsigned int)Qt::SolidPattern, 1},
        {(unsigned int)Qt::Dense1Pattern, 2},
        {(unsigned int)Qt::Dense2Pattern, 3},
        {(unsigned int)Qt::Dense3Pattern, 4},
        {(unsigned int)Qt::Dense4Pattern, 5},
        {(unsigned int)Qt::Dense5Pattern, 6},
        {(unsigned int)Qt::Dense6Pattern, 7},
        {(unsigned int)Qt::Dense7Pattern, 8},
        {(unsigned int)Qt::HorPattern, 9},
        {(unsigned int)Qt::VerPattern, 10},
        {(unsigned int)Qt::CrossPattern, 11},
        {(unsigned int)Qt::BDiagPattern, 12},
        {(unsigned int)Qt::FDiagPattern, 13},
        {(unsigned int)Qt::DiagCrossPattern, 14},
        {(unsigned int)Qt::LinearGradientPattern, 15},
        {(unsigned int)Qt::RadialGradientPattern, 16},
        {(unsigned int)Qt::ConicalGradientPattern, 17},
        {(unsigned int)Qt::TexturePattern, 24},
    };

    for (int i = 0; i < 19; ++i) {
        s_brush_style_lookup[brush_style_vals[i].key] =
            (unsigned int)brush_style_vals[i].val;
    }

    static KeyVal byte_order_vals[] = {
        {(unsigned int)QPixelFormat::LittleEndian, 0},
        {(unsigned int)QPixelFormat::BigEndian, 1},
        {(unsigned int)QPixelFormat::CurrentSystemEndian, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_byte_order_lookup[byte_order_vals[i].key] =
            (unsigned int)byte_order_vals[i].val;
    }

    static KeyVal capitalization_vals[] = {
        {(unsigned int)QFont::MixedCase, 0},
        {(unsigned int)QFont::AllUppercase, 1},
        {(unsigned int)QFont::AllLowercase, 2},
        {(unsigned int)QFont::SmallCaps, 3},
        {(unsigned int)QFont::Capitalize, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_capitalization_lookup[capitalization_vals[i].key] =
            (unsigned int)capitalization_vals[i].val;
    }

    static KeyVal case_sensitivity_vals[] = {
        {(unsigned int)Qt::CaseInsensitive, 0},
        {(unsigned int)Qt::CaseSensitive, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_case_sensitivity_lookup[case_sensitivity_vals[i].key] =
            (unsigned int)case_sensitivity_vals[i].val;
    }

    static KeyVal check_state_vals[] = {
        {(unsigned int)Qt::Unchecked, 0},
        {(unsigned int)Qt::PartiallyChecked, 1},
        {(unsigned int)Qt::Checked, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_check_state_lookup[check_state_vals[i].key] =
            (unsigned int)check_state_vals[i].val;
    }

    static KeyVal checksum_type_vals[] = {
        {(unsigned int)Qt::ChecksumIso3309, 0},
        {(unsigned int)Qt::ChecksumItuV41, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_checksum_type_lookup[checksum_type_vals[i].key] =
            (unsigned int)checksum_type_vals[i].val;
    }

    static KeyVal clip_operation_vals[] = {
        {(unsigned int)Qt::NoClip, 0},
        {(unsigned int)Qt::ReplaceClip, 1},
        {(unsigned int)Qt::IntersectClip, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_clip_operation_lookup[clip_operation_vals[i].key] =
            (unsigned int)clip_operation_vals[i].val;
    }

    static KeyVal color_group_vals[] = {
        {(unsigned int)QPalette::Active, 0},
        {(unsigned int)QPalette::Disabled, 1},
        {(unsigned int)QPalette::Inactive, 2},
        {(unsigned int)QPalette::NColorGroups, 3},
        {(unsigned int)QPalette::Current, 4},
        {(unsigned int)QPalette::All, 5},
        {(unsigned int)QPalette::Normal, 0},
    };

    for (int i = 0; i < 7; ++i) {
        s_color_group_lookup[color_group_vals[i].key] =
            (unsigned int)color_group_vals[i].val;
    }

    static KeyVal color_model_vals[] = {
        {(unsigned int)QPixelFormat::RGB, 0},
        {(unsigned int)QPixelFormat::BGR, 1},
        {(unsigned int)QPixelFormat::Indexed, 2},
        {(unsigned int)QPixelFormat::Grayscale, 3},
        {(unsigned int)QPixelFormat::CMYK, 4},
        {(unsigned int)QPixelFormat::HSL, 5},
        {(unsigned int)QPixelFormat::HSV, 6},
        {(unsigned int)QPixelFormat::YUV, 7},
        {(unsigned int)QPixelFormat::Alpha, 8},
    };

    for (int i = 0; i < 9; ++i) {
        s_color_model_lookup[color_model_vals[i].key] =
            (unsigned int)color_model_vals[i].val;
    }

    static KeyVal color_role_vals[] = {
        {(unsigned int)QPalette::WindowText, 0},
        {(unsigned int)QPalette::Button, 1},
        {(unsigned int)QPalette::Light, 2},
        {(unsigned int)QPalette::Midlight, 3},
        {(unsigned int)QPalette::Dark, 4},
        {(unsigned int)QPalette::Mid, 5},
        {(unsigned int)QPalette::Text, 6},
        {(unsigned int)QPalette::BrightText, 7},
        {(unsigned int)QPalette::ButtonText, 8},
        {(unsigned int)QPalette::Base, 9},
        {(unsigned int)QPalette::Window, 10},
        {(unsigned int)QPalette::Shadow, 11},
        {(unsigned int)QPalette::Highlight, 12},
        {(unsigned int)QPalette::HighlightedText, 13},
        {(unsigned int)QPalette::Link, 14},
        {(unsigned int)QPalette::LinkVisited, 15},
        {(unsigned int)QPalette::AlternateBase, 16},
        {(unsigned int)QPalette::NoRole, 17},
        {(unsigned int)QPalette::ToolTipBase, 18},
        {(unsigned int)QPalette::ToolTipText, 19},
        {(unsigned int)QPalette::NColorRoles, 20},
        {(unsigned int)QPalette::Foreground, 0},
        {(unsigned int)QPalette::Background, 10},
    };

    for (int i = 0; i < 23; ++i) {
        s_color_role_lookup[color_role_vals[i].key] =
            (unsigned int)color_role_vals[i].val;
    }

    static KeyVal color_space_vals[] = {
        {(unsigned int)QSurfaceFormat::DefaultColorSpace, 0},
        {(unsigned int)QSurfaceFormat::sRGBColorSpace, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_color_space_lookup[color_space_vals[i].key] =
            (unsigned int)color_space_vals[i].val;
    }

    static KeyVal color_spec_vals[] = {
        {(unsigned int)QApplication::NormalColor, 0},
        {(unsigned int)QApplication::CustomColor, 1},
        {(unsigned int)QApplication::ManyColor, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_color_spec_lookup[color_spec_vals[i].key] =
            (unsigned int)color_spec_vals[i].val;
    }

    static KeyVal complex_control_vals[] = {
        {(unsigned int)QStyle::CC_SpinBox, 0},
        {(unsigned int)QStyle::CC_ComboBox, 1},
        {(unsigned int)QStyle::CC_ScrollBar, 2},
        {(unsigned int)QStyle::CC_Slider, 3},
        {(unsigned int)QStyle::CC_ToolButton, 4},
        {(unsigned int)QStyle::CC_TitleBar, 5},
        {(unsigned int)QStyle::CC_Dial, 6},
        {(unsigned int)QStyle::CC_GroupBox, 7},
        {(unsigned int)QStyle::CC_MdiControls, 8},
        {(unsigned int)QStyle::CC_CustomBase, 4026531840},
    };

    for (int i = 0; i < 10; ++i) {
        s_complex_control_lookup[complex_control_vals[i].key] =
            (unsigned int)complex_control_vals[i].val;
    }

    static KeyVal composition_mode_vals[] = {
        {(unsigned int)QPainter::CompositionMode_SourceOver, 0},
        {(unsigned int)QPainter::CompositionMode_DestinationOver, 1},
        {(unsigned int)QPainter::CompositionMode_Clear, 2},
        {(unsigned int)QPainter::CompositionMode_Source, 3},
        {(unsigned int)QPainter::CompositionMode_Destination, 4},
        {(unsigned int)QPainter::CompositionMode_SourceIn, 5},
        {(unsigned int)QPainter::CompositionMode_DestinationIn, 6},
        {(unsigned int)QPainter::CompositionMode_SourceOut, 7},
        {(unsigned int)QPainter::CompositionMode_DestinationOut, 8},
        {(unsigned int)QPainter::CompositionMode_SourceAtop, 9},
        {(unsigned int)QPainter::CompositionMode_DestinationAtop, 10},
        {(unsigned int)QPainter::CompositionMode_Xor, 11},
        {(unsigned int)QPainter::CompositionMode_Plus, 12},
        {(unsigned int)QPainter::CompositionMode_Multiply, 13},
        {(unsigned int)QPainter::CompositionMode_Screen, 14},
        {(unsigned int)QPainter::CompositionMode_Overlay, 15},
        {(unsigned int)QPainter::CompositionMode_Darken, 16},
        {(unsigned int)QPainter::CompositionMode_Lighten, 17},
        {(unsigned int)QPainter::CompositionMode_ColorDodge, 18},
        {(unsigned int)QPainter::CompositionMode_ColorBurn, 19},
        {(unsigned int)QPainter::CompositionMode_HardLight, 20},
        {(unsigned int)QPainter::CompositionMode_SoftLight, 21},
        {(unsigned int)QPainter::CompositionMode_Difference, 22},
        {(unsigned int)QPainter::CompositionMode_Exclusion, 23},
        {(unsigned int)QPainter::RasterOp_SourceOrDestination, 24},
        {(unsigned int)QPainter::RasterOp_SourceAndDestination, 25},
        {(unsigned int)QPainter::RasterOp_SourceXorDestination, 26},
        {(unsigned int)QPainter::RasterOp_NotSourceAndNotDestination, 27},
        {(unsigned int)QPainter::RasterOp_NotSourceOrNotDestination, 28},
        {(unsigned int)QPainter::RasterOp_NotSourceXorDestination, 29},
        {(unsigned int)QPainter::RasterOp_NotSource, 30},
        {(unsigned int)QPainter::RasterOp_NotSourceAndDestination, 31},
        {(unsigned int)QPainter::RasterOp_SourceAndNotDestination, 32},
        {(unsigned int)QPainter::RasterOp_NotSourceOrDestination, 33},
        {(unsigned int)QPainter::RasterOp_SourceOrNotDestination, 34},
        {(unsigned int)QPainter::RasterOp_ClearDestination, 35},
        {(unsigned int)QPainter::RasterOp_SetDestination, 36},
        {(unsigned int)QPainter::RasterOp_NotDestination, 37},
    };

    for (int i = 0; i < 38; ++i) {
        s_composition_mode_lookup[composition_mode_vals[i].key] =
            (unsigned int)composition_mode_vals[i].val;
    }

    static KeyVal connection_type_vals[] = {
        {(unsigned int)Qt::AutoConnection, 0},
        {(unsigned int)Qt::DirectConnection, 1},
        {(unsigned int)Qt::QueuedConnection, 2},
        {(unsigned int)Qt::BlockingQueuedConnection, 3},
        {(unsigned int)Qt::UniqueConnection, 128},
    };

    for (int i = 0; i < 5; ++i) {
        s_connection_type_lookup[connection_type_vals[i].key] =
            (unsigned int)connection_type_vals[i].val;
    }

    static KeyVal contents_type_vals[] = {
        {(unsigned int)QStyle::CT_PushButton, 0},
        {(unsigned int)QStyle::CT_CheckBox, 1},
        {(unsigned int)QStyle::CT_RadioButton, 2},
        {(unsigned int)QStyle::CT_ToolButton, 3},
        {(unsigned int)QStyle::CT_ComboBox, 4},
        {(unsigned int)QStyle::CT_Splitter, 5},
        {(unsigned int)QStyle::CT_ProgressBar, 6},
        {(unsigned int)QStyle::CT_MenuItem, 7},
        {(unsigned int)QStyle::CT_MenuBarItem, 8},
        {(unsigned int)QStyle::CT_MenuBar, 9},
        {(unsigned int)QStyle::CT_Menu, 10},
        {(unsigned int)QStyle::CT_TabBarTab, 11},
        {(unsigned int)QStyle::CT_Slider, 12},
        {(unsigned int)QStyle::CT_ScrollBar, 13},
        {(unsigned int)QStyle::CT_LineEdit, 14},
        {(unsigned int)QStyle::CT_SpinBox, 15},
        {(unsigned int)QStyle::CT_SizeGrip, 16},
        {(unsigned int)QStyle::CT_TabWidget, 17},
        {(unsigned int)QStyle::CT_DialogButtons, 18},
        {(unsigned int)QStyle::CT_HeaderSection, 19},
        {(unsigned int)QStyle::CT_GroupBox, 20},
        {(unsigned int)QStyle::CT_MdiControls, 21},
        {(unsigned int)QStyle::CT_ItemViewItem, 22},
        {(unsigned int)QStyle::CT_CustomBase, 4026531840},
    };

    for (int i = 0; i < 24; ++i) {
        s_contents_type_lookup[contents_type_vals[i].key] =
            (unsigned int)contents_type_vals[i].val;
    }

    static KeyVal context_menu_policy_vals[] = {
        {(unsigned int)Qt::NoContextMenu, 0},
        {(unsigned int)Qt::DefaultContextMenu, 1},
        {(unsigned int)Qt::ActionsContextMenu, 2},
        {(unsigned int)Qt::CustomContextMenu, 3},
        {(unsigned int)Qt::PreventContextMenu, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_context_menu_policy_lookup[context_menu_policy_vals[i].key] =
            (unsigned int)context_menu_policy_vals[i].val;
    }

    static KeyVal control_element_vals[] = {
        {(unsigned int)QStyle::CE_PushButton, 0},
        {(unsigned int)QStyle::CE_PushButtonBevel, 1},
        {(unsigned int)QStyle::CE_PushButtonLabel, 2},
        {(unsigned int)QStyle::CE_CheckBox, 3},
        {(unsigned int)QStyle::CE_CheckBoxLabel, 4},
        {(unsigned int)QStyle::CE_RadioButton, 5},
        {(unsigned int)QStyle::CE_RadioButtonLabel, 6},
        {(unsigned int)QStyle::CE_TabBarTab, 7},
        {(unsigned int)QStyle::CE_TabBarTabShape, 8},
        {(unsigned int)QStyle::CE_TabBarTabLabel, 9},
        {(unsigned int)QStyle::CE_ProgressBar, 10},
        {(unsigned int)QStyle::CE_ProgressBarGroove, 11},
        {(unsigned int)QStyle::CE_ProgressBarContents, 12},
        {(unsigned int)QStyle::CE_ProgressBarLabel, 13},
        {(unsigned int)QStyle::CE_MenuItem, 14},
        {(unsigned int)QStyle::CE_MenuScroller, 15},
        {(unsigned int)QStyle::CE_MenuVMargin, 16},
        {(unsigned int)QStyle::CE_MenuHMargin, 17},
        {(unsigned int)QStyle::CE_MenuTearoff, 18},
        {(unsigned int)QStyle::CE_MenuEmptyArea, 19},
        {(unsigned int)QStyle::CE_MenuBarItem, 20},
        {(unsigned int)QStyle::CE_MenuBarEmptyArea, 21},
        {(unsigned int)QStyle::CE_ToolButtonLabel, 22},
        {(unsigned int)QStyle::CE_Header, 23},
        {(unsigned int)QStyle::CE_HeaderSection, 24},
        {(unsigned int)QStyle::CE_HeaderLabel, 25},
        {(unsigned int)QStyle::CE_ToolBoxTab, 26},
        {(unsigned int)QStyle::CE_SizeGrip, 27},
        {(unsigned int)QStyle::CE_Splitter, 28},
        {(unsigned int)QStyle::CE_RubberBand, 29},
        {(unsigned int)QStyle::CE_DockWidgetTitle, 30},
        {(unsigned int)QStyle::CE_ScrollBarAddLine, 31},
        {(unsigned int)QStyle::CE_ScrollBarSubLine, 32},
        {(unsigned int)QStyle::CE_ScrollBarAddPage, 33},
        {(unsigned int)QStyle::CE_ScrollBarSubPage, 34},
        {(unsigned int)QStyle::CE_ScrollBarSlider, 35},
        {(unsigned int)QStyle::CE_ScrollBarFirst, 36},
        {(unsigned int)QStyle::CE_ScrollBarLast, 37},
        {(unsigned int)QStyle::CE_FocusFrame, 38},
        {(unsigned int)QStyle::CE_ComboBoxLabel, 39},
        {(unsigned int)QStyle::CE_ToolBar, 40},
        {(unsigned int)QStyle::CE_ToolBoxTabShape, 41},
        {(unsigned int)QStyle::CE_ToolBoxTabLabel, 42},
        {(unsigned int)QStyle::CE_HeaderEmptyArea, 43},
        {(unsigned int)QStyle::CE_ColumnViewGrip, 44},
        {(unsigned int)QStyle::CE_ItemViewItem, 45},
        {(unsigned int)QStyle::CE_ShapedFrame, 46},
        {(unsigned int)QStyle::CE_CustomBase, 4026531840},
    };

    for (int i = 0; i < 48; ++i) {
        s_control_element_lookup[control_element_vals[i].key] =
            (unsigned int)control_element_vals[i].val;
    }

    static KeyVal control_type_vals[] = {
        {(unsigned int)QSizePolicy::DefaultType, 1},
        {(unsigned int)QSizePolicy::ButtonBox, 2},
        {(unsigned int)QSizePolicy::CheckBox, 4},
        {(unsigned int)QSizePolicy::ComboBox, 8},
        {(unsigned int)QSizePolicy::Frame, 16},
        {(unsigned int)QSizePolicy::GroupBox, 32},
        {(unsigned int)QSizePolicy::Label, 64},
        {(unsigned int)QSizePolicy::Line, 128},
        {(unsigned int)QSizePolicy::LineEdit, 256},
        {(unsigned int)QSizePolicy::PushButton, 512},
        {(unsigned int)QSizePolicy::RadioButton, 1024},
        {(unsigned int)QSizePolicy::Slider, 2048},
        {(unsigned int)QSizePolicy::SpinBox, 4096},
        {(unsigned int)QSizePolicy::TabWidget, 8192},
        {(unsigned int)QSizePolicy::ToolButton, 16384},
    };

    for (int i = 0; i < 15; ++i) {
        s_control_type_lookup[control_type_vals[i].key] =
            (unsigned int)control_type_vals[i].val;
    }

    static KeyVal coordinate_mode_vals[] = {
        {(unsigned int)QGradient::LogicalMode, 0},
        {(unsigned int)QGradient::StretchToDeviceMode, 1},
        {(unsigned int)QGradient::ObjectBoundingMode, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_coordinate_mode_lookup[coordinate_mode_vals[i].key] =
            (unsigned int)coordinate_mode_vals[i].val;
    }

    static KeyVal coordinate_system_vals[] = {
        {(unsigned int)Qt::DeviceCoordinates, 0},
        {(unsigned int)Qt::LogicalCoordinates, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_coordinate_system_lookup[coordinate_system_vals[i].key] =
            (unsigned int)coordinate_system_vals[i].val;
    }

    static KeyVal core_application_fix_me_enums_vals[] = {
        {(unsigned int)QCoreApplication::ApplicationFlags, 330498},
    };

    for (int i = 0; i < 1; ++i) {
        s_core_application_fix_me_enums_lookup
            [core_application_fix_me_enums_vals[i].key] =
                (unsigned int)core_application_fix_me_enums_vals[i].val;
    }

    static KeyVal corner_vals[] = {
        {(unsigned int)Qt::TopLeftCorner, 0},
        {(unsigned int)Qt::TopRightCorner, 1},
        {(unsigned int)Qt::BottomLeftCorner, 2},
        {(unsigned int)Qt::BottomRightCorner, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_corner_lookup[corner_vals[i].key] = (unsigned int)corner_vals[i].val;
    }

    static KeyVal cursor_move_style_vals[] = {
        {(unsigned int)Qt::LogicalMoveStyle, 0},
        {(unsigned int)Qt::VisualMoveStyle, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_cursor_move_style_lookup[cursor_move_style_vals[i].key] =
            (unsigned int)cursor_move_style_vals[i].val;
    }

    static KeyVal cursor_shape_vals[] = {
        {(unsigned int)Qt::ArrowCursor, 0},
        {(unsigned int)Qt::UpArrowCursor, 1},
        {(unsigned int)Qt::CrossCursor, 2},
        {(unsigned int)Qt::WaitCursor, 3},
        {(unsigned int)Qt::IBeamCursor, 4},
        {(unsigned int)Qt::SizeVerCursor, 5},
        {(unsigned int)Qt::SizeHorCursor, 6},
        {(unsigned int)Qt::SizeBDiagCursor, 7},
        {(unsigned int)Qt::SizeFDiagCursor, 8},
        {(unsigned int)Qt::SizeAllCursor, 9},
        {(unsigned int)Qt::BlankCursor, 10},
        {(unsigned int)Qt::SplitVCursor, 11},
        {(unsigned int)Qt::SplitHCursor, 12},
        {(unsigned int)Qt::PointingHandCursor, 13},
        {(unsigned int)Qt::ForbiddenCursor, 14},
        {(unsigned int)Qt::WhatsThisCursor, 15},
        {(unsigned int)Qt::BusyCursor, 16},
        {(unsigned int)Qt::OpenHandCursor, 17},
        {(unsigned int)Qt::ClosedHandCursor, 18},
        {(unsigned int)Qt::DragCopyCursor, 19},
        {(unsigned int)Qt::DragMoveCursor, 20},
        {(unsigned int)Qt::DragLinkCursor, 21},
        {(unsigned int)Qt::LastCursor, 21},
        {(unsigned int)Qt::BitmapCursor, 24},
        {(unsigned int)Qt::CustomCursor, 25},
    };

    for (int i = 0; i < 25; ++i) {
        s_cursor_shape_lookup[cursor_shape_vals[i].key] =
            (unsigned int)cursor_shape_vals[i].val;
    }

    static KeyVal date_format_vals[] = {
        {(unsigned int)Qt::TextDate, 0},
        {(unsigned int)Qt::ISODate, 1},
        {(unsigned int)Qt::SystemLocaleDate, 2},
        {(unsigned int)Qt::LocalDate, 2},
        {(unsigned int)Qt::LocaleDate, 3},
        {(unsigned int)Qt::SystemLocaleShortDate, 4},
        {(unsigned int)Qt::SystemLocaleLongDate, 5},
        {(unsigned int)Qt::DefaultLocaleShortDate, 6},
        {(unsigned int)Qt::DefaultLocaleLongDate, 7},
        {(unsigned int)Qt::ISODateWithMs, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_date_format_lookup[date_format_vals[i].key] =
            (unsigned int)date_format_vals[i].val;
    }

    static KeyVal day_of_week_vals[] = {
        {(unsigned int)Qt::Monday, 1},    {(unsigned int)Qt::Tuesday, 2},
        {(unsigned int)Qt::Wednesday, 3}, {(unsigned int)Qt::Thursday, 4},
        {(unsigned int)Qt::Friday, 5},    {(unsigned int)Qt::Saturday, 6},
        {(unsigned int)Qt::Sunday, 7},
    };

    for (int i = 0; i < 7; ++i) {
        s_day_of_week_lookup[day_of_week_vals[i].key] =
            (unsigned int)day_of_week_vals[i].val;
    }

    static KeyVal dirty_flag_vals[] = {
        {(unsigned int)QPaintEngine::DirtyPen, 1},
        {(unsigned int)QPaintEngine::DirtyBrush, 2},
        {(unsigned int)QPaintEngine::DirtyBrushOrigin, 4},
        {(unsigned int)QPaintEngine::DirtyFont, 8},
        {(unsigned int)QPaintEngine::DirtyBackground, 16},
        {(unsigned int)QPaintEngine::DirtyBackgroundMode, 32},
        {(unsigned int)QPaintEngine::DirtyTransform, 64},
        {(unsigned int)QPaintEngine::DirtyClipRegion, 128},
        {(unsigned int)QPaintEngine::DirtyClipPath, 256},
        {(unsigned int)QPaintEngine::DirtyHints, 512},
        {(unsigned int)QPaintEngine::DirtyCompositionMode, 1024},
        {(unsigned int)QPaintEngine::DirtyClipEnabled, 2048},
        {(unsigned int)QPaintEngine::DirtyOpacity, 4096},
        {(unsigned int)QPaintEngine::AllDirty, 65535},
    };

    for (int i = 0; i < 14; ++i) {
        s_dirty_flag_lookup[dirty_flag_vals[i].key] =
            (unsigned int)dirty_flag_vals[i].val;
    }

    static KeyVal dock_widget_area_vals[] = {
        {(unsigned int)Qt::LeftDockWidgetArea, 1},
        {(unsigned int)Qt::RightDockWidgetArea, 2},
        {(unsigned int)Qt::TopDockWidgetArea, 4},
        {(unsigned int)Qt::BottomDockWidgetArea, 8},
        {(unsigned int)Qt::DockWidgetArea_Mask, 15},
        {(unsigned int)Qt::AllDockWidgetAreas, 15},
        {(unsigned int)Qt::NoDockWidgetArea, 0},
    };

    for (int i = 0; i < 7; ++i) {
        s_dock_widget_area_lookup[dock_widget_area_vals[i].key] =
            (unsigned int)dock_widget_area_vals[i].val;
    }

    static KeyVal dock_widget_area_sizes_vals[] = {
        {(unsigned int)Qt::NDockWidgetAreas, 4},
    };

    for (int i = 0; i < 1; ++i) {
        s_dock_widget_area_sizes_lookup[dock_widget_area_sizes_vals[i].key] =
            (unsigned int)dock_widget_area_sizes_vals[i].val;
    }

    static KeyVal drop_action_vals[] = {
        {(unsigned int)Qt::CopyAction, 1},
        {(unsigned int)Qt::MoveAction, 2},
        {(unsigned int)Qt::LinkAction, 4},
        {(unsigned int)Qt::ActionMask, 255},
        {(unsigned int)Qt::TargetMoveAction, 32770},
        {(unsigned int)Qt::IgnoreAction, 0},
    };

    for (int i = 0; i < 6; ++i) {
        s_drop_action_lookup[drop_action_vals[i].key] =
            (unsigned int)drop_action_vals[i].val;
    }

    static KeyVal echo_mode_vals[] = {
        {(unsigned int)QLineEdit::Normal, 0},
        {(unsigned int)QLineEdit::NoEcho, 1},
        {(unsigned int)QLineEdit::Password, 2},
        {(unsigned int)QLineEdit::PasswordEchoOnEdit, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_echo_mode_lookup[echo_mode_vals[i].key] =
            (unsigned int)echo_mode_vals[i].val;
    }

    static KeyVal edge_vals[] = {
        {(unsigned int)Qt::TopEdge, 1},
        {(unsigned int)Qt::LeftEdge, 2},
        {(unsigned int)Qt::RightEdge, 4},
        {(unsigned int)Qt::BottomEdge, 8},
    };

    for (int i = 0; i < 4; ++i) {
        s_edge_lookup[edge_vals[i].key] = (unsigned int)edge_vals[i].val;
    }

    static KeyVal enter_key_type_vals[] = {
        {(unsigned int)Qt::EnterKeyDefault, 0},
        {(unsigned int)Qt::EnterKeyReturn, 1},
        {(unsigned int)Qt::EnterKeyDone, 2},
        {(unsigned int)Qt::EnterKeyGo, 3},
        {(unsigned int)Qt::EnterKeySend, 4},
        {(unsigned int)Qt::EnterKeySearch, 5},
        {(unsigned int)Qt::EnterKeyNext, 6},
        {(unsigned int)Qt::EnterKeyPrevious, 7},
    };

    for (int i = 0; i < 8; ++i) {
        s_enter_key_type_lookup[enter_key_type_vals[i].key] =
            (unsigned int)enter_key_type_vals[i].val;
    }

    static KeyVal event_priority_vals[] = {
        {(unsigned int)Qt::HighEventPriority, 1},
        {(unsigned int)Qt::NormalEventPriority, 0},
        {(unsigned int)Qt::LowEventPriority, 4294967295},
    };

    for (int i = 0; i < 3; ++i) {
        s_event_priority_lookup[event_priority_vals[i].key] =
            (unsigned int)event_priority_vals[i].val;
    }

    static KeyVal fill_rule_vals[] = {
        {(unsigned int)Qt::OddEvenFill, 0},
        {(unsigned int)Qt::WindingFill, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_fill_rule_lookup[fill_rule_vals[i].key] =
            (unsigned int)fill_rule_vals[i].val;
    }

    static KeyVal find_child_option_vals[] = {
        {(unsigned int)Qt::FindDirectChildrenOnly, 0},
        {(unsigned int)Qt::FindChildrenRecursively, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_find_child_option_lookup[find_child_option_vals[i].key] =
            (unsigned int)find_child_option_vals[i].val;
    }

    static KeyVal focus_policy_vals[] = {
        {(unsigned int)Qt::NoFocus, 0},     {(unsigned int)Qt::TabFocus, 1},
        {(unsigned int)Qt::ClickFocus, 2},  {(unsigned int)Qt::StrongFocus, 11},
        {(unsigned int)Qt::WheelFocus, 15},
    };

    for (int i = 0; i < 5; ++i) {
        s_focus_policy_lookup[focus_policy_vals[i].key] =
            (unsigned int)focus_policy_vals[i].val;
    }

    static KeyVal focus_reason_vals[] = {
        {(unsigned int)Qt::MouseFocusReason, 0},
        {(unsigned int)Qt::TabFocusReason, 1},
        {(unsigned int)Qt::BacktabFocusReason, 2},
        {(unsigned int)Qt::ActiveWindowFocusReason, 3},
        {(unsigned int)Qt::PopupFocusReason, 4},
        {(unsigned int)Qt::ShortcutFocusReason, 5},
        {(unsigned int)Qt::MenuBarFocusReason, 6},
        {(unsigned int)Qt::OtherFocusReason, 7},
        {(unsigned int)Qt::NoFocusReason, 8},
    };

    for (int i = 0; i < 9; ++i) {
        s_focus_reason_lookup[focus_reason_vals[i].key] =
            (unsigned int)focus_reason_vals[i].val;
    }

    static KeyVal format_vals[] = {
        {(unsigned int)QImage::Format_Invalid, 0},
        {(unsigned int)QImage::Format_Mono, 1},
        {(unsigned int)QImage::Format_MonoLSB, 2},
        {(unsigned int)QImage::Format_Indexed8, 3},
        {(unsigned int)QImage::Format_RGB32, 4},
        {(unsigned int)QImage::Format_ARGB32, 5},
        {(unsigned int)QImage::Format_ARGB32_Premultiplied, 6},
        {(unsigned int)QImage::Format_RGB16, 7},
        {(unsigned int)QImage::Format_ARGB8565_Premultiplied, 8},
        {(unsigned int)QImage::Format_RGB666, 9},
        {(unsigned int)QImage::Format_ARGB6666_Premultiplied, 10},
        {(unsigned int)QImage::Format_RGB555, 11},
        {(unsigned int)QImage::Format_ARGB8555_Premultiplied, 12},
        {(unsigned int)QImage::Format_RGB888, 13},
        {(unsigned int)QImage::Format_RGB444, 14},
        {(unsigned int)QImage::Format_ARGB4444_Premultiplied, 15},
        {(unsigned int)QImage::Format_RGBX8888, 16},
        {(unsigned int)QImage::Format_RGBA8888, 17},
        {(unsigned int)QImage::Format_RGBA8888_Premultiplied, 18},
        {(unsigned int)QImage::Format_BGR30, 19},
        {(unsigned int)QImage::Format_A2BGR30_Premultiplied, 20},
        {(unsigned int)QImage::Format_RGB30, 21},
        {(unsigned int)QImage::Format_A2RGB30_Premultiplied, 22},
        {(unsigned int)QImage::Format_Alpha8, 23},
        {(unsigned int)QImage::Format_Grayscale8, 24},
        {(unsigned int)QImage::NImageFormats, 25},
    };

    for (int i = 0; i < 26; ++i) {
        s_format_lookup[format_vals[i].key] = (unsigned int)format_vals[i].val;
    }

    static KeyVal format_option_vals[] = {
        {(unsigned int)QSurfaceFormat::StereoBuffers, 1},
        {(unsigned int)QSurfaceFormat::DebugContext, 2},
        {(unsigned int)QSurfaceFormat::DeprecatedFunctions, 4},
        {(unsigned int)QSurfaceFormat::ResetNotification, 8},
    };

    for (int i = 0; i < 4; ++i) {
        s_format_option_lookup[format_option_vals[i].key] =
            (unsigned int)format_option_vals[i].val;
    }

    static KeyVal gesture_flag_vals[] = {
        {(unsigned int)Qt::DontStartGestureOnChildren, 1},
        {(unsigned int)Qt::ReceivePartialGestures, 2},
        {(unsigned int)Qt::IgnoredGesturesPropagateToParent, 4},
    };

    for (int i = 0; i < 3; ++i) {
        s_gesture_flag_lookup[gesture_flag_vals[i].key] =
            (unsigned int)gesture_flag_vals[i].val;
    }

    static KeyVal gesture_state_vals[] = {
        {(unsigned int)Qt::NoGesture, 0},
        {(unsigned int)Qt::GestureStarted, 1},
        {(unsigned int)Qt::GestureUpdated, 2},
        {(unsigned int)Qt::GestureFinished, 3},
        {(unsigned int)Qt::GestureCanceled, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_gesture_state_lookup[gesture_state_vals[i].key] =
            (unsigned int)gesture_state_vals[i].val;
    }

    static KeyVal gesture_type_vals[] = {
        {(unsigned int)Qt::TapGesture, 1},
        {(unsigned int)Qt::TapAndHoldGesture, 2},
        {(unsigned int)Qt::PanGesture, 3},
        {(unsigned int)Qt::PinchGesture, 4},
        {(unsigned int)Qt::SwipeGesture, 5},
        {(unsigned int)Qt::CustomGesture, 256},
        {(unsigned int)Qt::LastGestureType, 4294967295},
    };

    for (int i = 0; i < 7; ++i) {
        s_gesture_type_lookup[gesture_type_vals[i].key] =
            (unsigned int)gesture_type_vals[i].val;
    }

    static KeyVal global_color_vals[] = {
        {(unsigned int)Qt::color0, 0},      {(unsigned int)Qt::color1, 1},
        {(unsigned int)Qt::black, 2},       {(unsigned int)Qt::white, 3},
        {(unsigned int)Qt::darkGray, 4},    {(unsigned int)Qt::gray, 5},
        {(unsigned int)Qt::lightGray, 6},   {(unsigned int)Qt::red, 7},
        {(unsigned int)Qt::green, 8},       {(unsigned int)Qt::blue, 9},
        {(unsigned int)Qt::cyan, 10},       {(unsigned int)Qt::magenta, 11},
        {(unsigned int)Qt::yellow, 12},     {(unsigned int)Qt::darkRed, 13},
        {(unsigned int)Qt::darkGreen, 14},  {(unsigned int)Qt::darkBlue, 15},
        {(unsigned int)Qt::darkCyan, 16},   {(unsigned int)Qt::darkMagenta, 17},
        {(unsigned int)Qt::darkYellow, 18}, {(unsigned int)Qt::transparent, 19},
    };

    for (int i = 0; i < 20; ++i) {
        s_global_color_lookup[global_color_vals[i].key] =
            (unsigned int)global_color_vals[i].val;
    }

    static KeyVal hinting_preference_vals[] = {
        {(unsigned int)QFont::PreferDefaultHinting, 0},
        {(unsigned int)QFont::PreferNoHinting, 1},
        {(unsigned int)QFont::PreferVerticalHinting, 2},
        {(unsigned int)QFont::PreferFullHinting, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_hinting_preference_lookup[hinting_preference_vals[i].key] =
            (unsigned int)hinting_preference_vals[i].val;
    }

    static KeyVal hit_test_accuracy_vals[] = {
        {(unsigned int)Qt::ExactHit, 0},
        {(unsigned int)Qt::FuzzyHit, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_hit_test_accuracy_lookup[hit_test_accuracy_vals[i].key] =
            (unsigned int)hit_test_accuracy_vals[i].val;
    }

    static KeyVal image_conversion_flag_vals[] = {
        {(unsigned int)Qt::ColorMode_Mask, 3},
        {(unsigned int)Qt::AutoColor, 0},
        {(unsigned int)Qt::ColorOnly, 3},
        {(unsigned int)Qt::MonoOnly, 2},
        {(unsigned int)Qt::AlphaDither_Mask, 12},
        {(unsigned int)Qt::ThresholdAlphaDither, 0},
        {(unsigned int)Qt::OrderedAlphaDither, 4},
        {(unsigned int)Qt::DiffuseAlphaDither, 8},
        {(unsigned int)Qt::NoAlpha, 12},
        {(unsigned int)Qt::Dither_Mask, 48},
        {(unsigned int)Qt::DiffuseDither, 0},
        {(unsigned int)Qt::OrderedDither, 16},
        {(unsigned int)Qt::ThresholdDither, 32},
        {(unsigned int)Qt::DitherMode_Mask, 192},
        {(unsigned int)Qt::AutoDither, 0},
        {(unsigned int)Qt::PreferDither, 64},
        {(unsigned int)Qt::AvoidDither, 128},
        {(unsigned int)Qt::NoOpaqueDetection, 256},
        {(unsigned int)Qt::NoFormatConversion, 512},
    };

    for (int i = 0; i < 19; ++i) {
        s_image_conversion_flag_lookup[image_conversion_flag_vals[i].key] =
            (unsigned int)image_conversion_flag_vals[i].val;
    }

    static KeyVal initialization_vals[] = {
        {(unsigned int)Qt::Uninitialized, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_initialization_lookup[initialization_vals[i].key] =
            (unsigned int)initialization_vals[i].val;
    }

    static KeyVal input_method_hint_vals[] = {
        {(unsigned int)Qt::ImhNone, 0},
        {(unsigned int)Qt::ImhHiddenText, 1},
        {(unsigned int)Qt::ImhSensitiveData, 2},
        {(unsigned int)Qt::ImhNoAutoUppercase, 4},
        {(unsigned int)Qt::ImhPreferNumbers, 8},
        {(unsigned int)Qt::ImhPreferUppercase, 16},
        {(unsigned int)Qt::ImhPreferLowercase, 32},
        {(unsigned int)Qt::ImhNoPredictiveText, 64},
        {(unsigned int)Qt::ImhDate, 128},
        {(unsigned int)Qt::ImhTime, 256},
        {(unsigned int)Qt::ImhPreferLatin, 512},
        {(unsigned int)Qt::ImhMultiLine, 1024},
        {(unsigned int)Qt::ImhNoEditMenu, 2048},
        {(unsigned int)Qt::ImhNoTextHandles, 4096},
        {(unsigned int)Qt::ImhDigitsOnly, 65536},
        {(unsigned int)Qt::ImhFormattedNumbersOnly, 131072},
        {(unsigned int)Qt::ImhUppercaseOnly, 262144},
        {(unsigned int)Qt::ImhLowercaseOnly, 524288},
        {(unsigned int)Qt::ImhDialableCharactersOnly, 1048576},
        {(unsigned int)Qt::ImhEmailCharactersOnly, 2097152},
        {(unsigned int)Qt::ImhUrlCharactersOnly, 4194304},
        {(unsigned int)Qt::ImhLatinOnly, 8388608},
        {(unsigned int)Qt::ImhExclusiveInputMask, 4294901760},
    };

    for (int i = 0; i < 23; ++i) {
        s_input_method_hint_lookup[input_method_hint_vals[i].key] =
            (unsigned int)input_method_hint_vals[i].val;
    }

    static KeyVal input_method_query_vals[] = {
        {(unsigned int)Qt::ImEnabled, 1},
        {(unsigned int)Qt::ImCursorRectangle, 2},
        {(unsigned int)Qt::ImMicroFocus, 2},
        {(unsigned int)Qt::ImFont, 4},
        {(unsigned int)Qt::ImCursorPosition, 8},
        {(unsigned int)Qt::ImSurroundingText, 16},
        {(unsigned int)Qt::ImCurrentSelection, 32},
        {(unsigned int)Qt::ImMaximumTextLength, 64},
        {(unsigned int)Qt::ImAnchorPosition, 128},
        {(unsigned int)Qt::ImHints, 256},
        {(unsigned int)Qt::ImPreferredLanguage, 512},
        {(unsigned int)Qt::ImAbsolutePosition, 1024},
        {(unsigned int)Qt::ImTextBeforeCursor, 2048},
        {(unsigned int)Qt::ImTextAfterCursor, 4096},
        {(unsigned int)Qt::ImEnterKeyType, 8192},
        {(unsigned int)Qt::ImAnchorRectangle, 16384},
        {(unsigned int)Qt::ImInputItemClipRectangle, 32768},
        {(unsigned int)Qt::ImPlatformData, 2147483648},
        {(unsigned int)Qt::ImQueryInput, 16570},
        {(unsigned int)Qt::ImQueryAll, 4294967295},
    };

    for (int i = 0; i < 20; ++i) {
        s_input_method_query_lookup[input_method_query_vals[i].key] =
            (unsigned int)input_method_query_vals[i].val;
    }

    static KeyVal interpolation_mode_vals[] = {
        {(unsigned int)QGradient::ColorInterpolation, 0},
        {(unsigned int)QGradient::ComponentInterpolation, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_interpolation_mode_lookup[interpolation_mode_vals[i].key] =
            (unsigned int)interpolation_mode_vals[i].val;
    }

    static KeyVal intersect_type_vals[] = {
        {(unsigned int)QLineF::NoIntersection, 0},
        {(unsigned int)QLineF::BoundedIntersection, 1},
        {(unsigned int)QLineF::UnboundedIntersection, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_intersect_type_lookup[intersect_type_vals[i].key] =
            (unsigned int)intersect_type_vals[i].val;
    }

    static KeyVal invert_mode_vals[] = {
        {(unsigned int)QImage::InvertRgb, 0},
        {(unsigned int)QImage::InvertRgba, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_invert_mode_lookup[invert_mode_vals[i].key] =
            (unsigned int)invert_mode_vals[i].val;
    }

    static KeyVal item_data_role_vals[] = {
        {(unsigned int)Qt::DisplayRole, 0},
        {(unsigned int)Qt::DecorationRole, 1},
        {(unsigned int)Qt::EditRole, 2},
        {(unsigned int)Qt::ToolTipRole, 3},
        {(unsigned int)Qt::StatusTipRole, 4},
        {(unsigned int)Qt::WhatsThisRole, 5},
        {(unsigned int)Qt::FontRole, 6},
        {(unsigned int)Qt::TextAlignmentRole, 7},
        {(unsigned int)Qt::BackgroundColorRole, 8},
        {(unsigned int)Qt::BackgroundRole, 8},
        {(unsigned int)Qt::TextColorRole, 9},
        {(unsigned int)Qt::ForegroundRole, 9},
        {(unsigned int)Qt::CheckStateRole, 10},
        {(unsigned int)Qt::AccessibleTextRole, 11},
        {(unsigned int)Qt::AccessibleDescriptionRole, 12},
        {(unsigned int)Qt::SizeHintRole, 13},
        {(unsigned int)Qt::InitialSortOrderRole, 14},
        {(unsigned int)Qt::DisplayPropertyRole, 27},
        {(unsigned int)Qt::DecorationPropertyRole, 28},
        {(unsigned int)Qt::ToolTipPropertyRole, 29},
        {(unsigned int)Qt::StatusTipPropertyRole, 30},
        {(unsigned int)Qt::WhatsThisPropertyRole, 31},
        {(unsigned int)Qt::UserRole, 256},
    };

    for (int i = 0; i < 23; ++i) {
        s_item_data_role_lookup[item_data_role_vals[i].key] =
            (unsigned int)item_data_role_vals[i].val;
    }

    static KeyVal item_flag_vals[] = {
        {(unsigned int)Qt::NoItemFlags, 0},
        {(unsigned int)Qt::ItemIsSelectable, 1},
        {(unsigned int)Qt::ItemIsEditable, 2},
        {(unsigned int)Qt::ItemIsDragEnabled, 4},
        {(unsigned int)Qt::ItemIsDropEnabled, 8},
        {(unsigned int)Qt::ItemIsUserCheckable, 16},
        {(unsigned int)Qt::ItemIsEnabled, 32},
        {(unsigned int)Qt::ItemIsAutoTristate, 64},
        {(unsigned int)Qt::ItemIsTristate, 64},
        {(unsigned int)Qt::ItemNeverHasChildren, 128},
        {(unsigned int)Qt::ItemIsUserTristate, 256},
    };

    for (int i = 0; i < 11; ++i) {
        s_item_flag_lookup[item_flag_vals[i].key] =
            (unsigned int)item_flag_vals[i].val;
    }

    static KeyVal item_selection_mode_vals[] = {
        {(unsigned int)Qt::ContainsItemShape, 0},
        {(unsigned int)Qt::IntersectsItemShape, 1},
        {(unsigned int)Qt::ContainsItemBoundingRect, 2},
        {(unsigned int)Qt::IntersectsItemBoundingRect, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_item_selection_mode_lookup[item_selection_mode_vals[i].key] =
            (unsigned int)item_selection_mode_vals[i].val;
    }

    static KeyVal item_selection_operation_vals[] = {
        {(unsigned int)Qt::ReplaceSelection, 0},
        {(unsigned int)Qt::AddToSelection, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_item_selection_operation_lookup[item_selection_operation_vals[i]
                                              .key] =
            (unsigned int)item_selection_operation_vals[i].val;
    }

    static KeyVal item_type_vals[] = {
        {(unsigned int)QListWidgetItem::Type, 0},
        {(unsigned int)QListWidgetItem::UserType, 1000},
    };

    for (int i = 0; i < 2; ++i) {
        s_item_type_lookup[item_type_vals[i].key] =
            (unsigned int)item_type_vals[i].val;
    }

    static KeyVal key_vals[] = {
        {(unsigned int)Qt::Key_Escape, 16777216},
        {(unsigned int)Qt::Key_Tab, 16777217},
        {(unsigned int)Qt::Key_Backtab, 16777218},
        {(unsigned int)Qt::Key_Backspace, 16777219},
        {(unsigned int)Qt::Key_Return, 16777220},
        {(unsigned int)Qt::Key_Enter, 16777221},
        {(unsigned int)Qt::Key_Insert, 16777222},
        {(unsigned int)Qt::Key_Delete, 16777223},
        {(unsigned int)Qt::Key_Pause, 16777224},
        {(unsigned int)Qt::Key_Print, 16777225},
        {(unsigned int)Qt::Key_SysReq, 16777226},
        {(unsigned int)Qt::Key_Clear, 16777227},
        {(unsigned int)Qt::Key_Home, 16777232},
        {(unsigned int)Qt::Key_End, 16777233},
        {(unsigned int)Qt::Key_Left, 16777234},
        {(unsigned int)Qt::Key_Up, 16777235},
        {(unsigned int)Qt::Key_Right, 16777236},
        {(unsigned int)Qt::Key_Down, 16777237},
        {(unsigned int)Qt::Key_PageUp, 16777238},
        {(unsigned int)Qt::Key_PageDown, 16777239},
        {(unsigned int)Qt::Key_Shift, 16777248},
        {(unsigned int)Qt::Key_Control, 16777249},
        {(unsigned int)Qt::Key_Meta, 16777250},
        {(unsigned int)Qt::Key_Alt, 16777251},
        {(unsigned int)Qt::Key_CapsLock, 16777252},
        {(unsigned int)Qt::Key_NumLock, 16777253},
        {(unsigned int)Qt::Key_ScrollLock, 16777254},
        {(unsigned int)Qt::Key_F1, 16777264},
        {(unsigned int)Qt::Key_F2, 16777265},
        {(unsigned int)Qt::Key_F3, 16777266},
        {(unsigned int)Qt::Key_F4, 16777267},
        {(unsigned int)Qt::Key_F5, 16777268},
        {(unsigned int)Qt::Key_F6, 16777269},
        {(unsigned int)Qt::Key_F7, 16777270},
        {(unsigned int)Qt::Key_F8, 16777271},
        {(unsigned int)Qt::Key_F9, 16777272},
        {(unsigned int)Qt::Key_F10, 16777273},
        {(unsigned int)Qt::Key_F11, 16777274},
        {(unsigned int)Qt::Key_F12, 16777275},
        {(unsigned int)Qt::Key_F13, 16777276},
        {(unsigned int)Qt::Key_F14, 16777277},
        {(unsigned int)Qt::Key_F15, 16777278},
        {(unsigned int)Qt::Key_F16, 16777279},
        {(unsigned int)Qt::Key_F17, 16777280},
        {(unsigned int)Qt::Key_F18, 16777281},
        {(unsigned int)Qt::Key_F19, 16777282},
        {(unsigned int)Qt::Key_F20, 16777283},
        {(unsigned int)Qt::Key_F21, 16777284},
        {(unsigned int)Qt::Key_F22, 16777285},
        {(unsigned int)Qt::Key_F23, 16777286},
        {(unsigned int)Qt::Key_F24, 16777287},
        {(unsigned int)Qt::Key_F25, 16777288},
        {(unsigned int)Qt::Key_F26, 16777289},
        {(unsigned int)Qt::Key_F27, 16777290},
        {(unsigned int)Qt::Key_F28, 16777291},
        {(unsigned int)Qt::Key_F29, 16777292},
        {(unsigned int)Qt::Key_F30, 16777293},
        {(unsigned int)Qt::Key_F31, 16777294},
        {(unsigned int)Qt::Key_F32, 16777295},
        {(unsigned int)Qt::Key_F33, 16777296},
        {(unsigned int)Qt::Key_F34, 16777297},
        {(unsigned int)Qt::Key_F35, 16777298},
        {(unsigned int)Qt::Key_Super_L, 16777299},
        {(unsigned int)Qt::Key_Super_R, 16777300},
        {(unsigned int)Qt::Key_Menu, 16777301},
        {(unsigned int)Qt::Key_Hyper_L, 16777302},
        {(unsigned int)Qt::Key_Hyper_R, 16777303},
        {(unsigned int)Qt::Key_Help, 16777304},
        {(unsigned int)Qt::Key_Direction_L, 16777305},
        {(unsigned int)Qt::Key_Direction_R, 16777312},
        {(unsigned int)Qt::Key_Space, 32},
        {(unsigned int)Qt::Key_Any, 32},
        {(unsigned int)Qt::Key_Exclam, 33},
        {(unsigned int)Qt::Key_QuoteDbl, 34},
        {(unsigned int)Qt::Key_NumberSign, 35},
        {(unsigned int)Qt::Key_Dollar, 36},
        {(unsigned int)Qt::Key_Percent, 37},
        {(unsigned int)Qt::Key_Ampersand, 38},
        {(unsigned int)Qt::Key_Apostrophe, 39},
        {(unsigned int)Qt::Key_ParenLeft, 40},
        {(unsigned int)Qt::Key_ParenRight, 41},
        {(unsigned int)Qt::Key_Asterisk, 42},
        {(unsigned int)Qt::Key_Plus, 43},
        {(unsigned int)Qt::Key_Comma, 44},
        {(unsigned int)Qt::Key_Minus, 45},
        {(unsigned int)Qt::Key_Period, 46},
        {(unsigned int)Qt::Key_Slash, 47},
        {(unsigned int)Qt::Key_0, 48},
        {(unsigned int)Qt::Key_1, 49},
        {(unsigned int)Qt::Key_2, 50},
        {(unsigned int)Qt::Key_3, 51},
        {(unsigned int)Qt::Key_4, 52},
        {(unsigned int)Qt::Key_5, 53},
        {(unsigned int)Qt::Key_6, 54},
        {(unsigned int)Qt::Key_7, 55},
        {(unsigned int)Qt::Key_8, 56},
        {(unsigned int)Qt::Key_9, 57},
        {(unsigned int)Qt::Key_Colon, 58},
        {(unsigned int)Qt::Key_Semicolon, 59},
        {(unsigned int)Qt::Key_Less, 60},
        {(unsigned int)Qt::Key_Equal, 61},
        {(unsigned int)Qt::Key_Greater, 62},
        {(unsigned int)Qt::Key_Question, 63},
        {(unsigned int)Qt::Key_At, 64},
        {(unsigned int)Qt::Key_A, 65},
        {(unsigned int)Qt::Key_B, 66},
        {(unsigned int)Qt::Key_C, 67},
        {(unsigned int)Qt::Key_D, 68},
        {(unsigned int)Qt::Key_E, 69},
        {(unsigned int)Qt::Key_F, 70},
        {(unsigned int)Qt::Key_G, 71},
        {(unsigned int)Qt::Key_H, 72},
        {(unsigned int)Qt::Key_I, 73},
        {(unsigned int)Qt::Key_J, 74},
        {(unsigned int)Qt::Key_K, 75},
        {(unsigned int)Qt::Key_L, 76},
        {(unsigned int)Qt::Key_M, 77},
        {(unsigned int)Qt::Key_N, 78},
        {(unsigned int)Qt::Key_O, 79},
        {(unsigned int)Qt::Key_P, 80},
        {(unsigned int)Qt::Key_Q, 81},
        {(unsigned int)Qt::Key_R, 82},
        {(unsigned int)Qt::Key_S, 83},
        {(unsigned int)Qt::Key_T, 84},
        {(unsigned int)Qt::Key_U, 85},
        {(unsigned int)Qt::Key_V, 86},
        {(unsigned int)Qt::Key_W, 87},
        {(unsigned int)Qt::Key_X, 88},
        {(unsigned int)Qt::Key_Y, 89},
        {(unsigned int)Qt::Key_Z, 90},
        {(unsigned int)Qt::Key_BracketLeft, 91},
        {(unsigned int)Qt::Key_Backslash, 92},
        {(unsigned int)Qt::Key_BracketRight, 93},
        {(unsigned int)Qt::Key_AsciiCircum, 94},
        {(unsigned int)Qt::Key_Underscore, 95},
        {(unsigned int)Qt::Key_QuoteLeft, 96},
        {(unsigned int)Qt::Key_BraceLeft, 123},
        {(unsigned int)Qt::Key_Bar, 124},
        {(unsigned int)Qt::Key_BraceRight, 125},
        {(unsigned int)Qt::Key_AsciiTilde, 126},
        {(unsigned int)Qt::Key_nobreakspace, 160},
        {(unsigned int)Qt::Key_exclamdown, 161},
        {(unsigned int)Qt::Key_cent, 162},
        {(unsigned int)Qt::Key_sterling, 163},
        {(unsigned int)Qt::Key_currency, 164},
        {(unsigned int)Qt::Key_yen, 165},
        {(unsigned int)Qt::Key_brokenbar, 166},
        {(unsigned int)Qt::Key_section, 167},
        {(unsigned int)Qt::Key_diaeresis, 168},
        {(unsigned int)Qt::Key_copyright, 169},
        {(unsigned int)Qt::Key_ordfeminine, 170},
        {(unsigned int)Qt::Key_guillemotleft, 171},
        {(unsigned int)Qt::Key_notsign, 172},
        {(unsigned int)Qt::Key_hyphen, 173},
        {(unsigned int)Qt::Key_registered, 174},
        {(unsigned int)Qt::Key_macron, 175},
        {(unsigned int)Qt::Key_degree, 176},
        {(unsigned int)Qt::Key_plusminus, 177},
        {(unsigned int)Qt::Key_twosuperior, 178},
        {(unsigned int)Qt::Key_threesuperior, 179},
        {(unsigned int)Qt::Key_acute, 180},
        {(unsigned int)Qt::Key_mu, 181},
        {(unsigned int)Qt::Key_paragraph, 182},
        {(unsigned int)Qt::Key_periodcentered, 183},
        {(unsigned int)Qt::Key_cedilla, 184},
        {(unsigned int)Qt::Key_onesuperior, 185},
        {(unsigned int)Qt::Key_masculine, 186},
        {(unsigned int)Qt::Key_guillemotright, 187},
        {(unsigned int)Qt::Key_onequarter, 188},
        {(unsigned int)Qt::Key_onehalf, 189},
        {(unsigned int)Qt::Key_threequarters, 190},
        {(unsigned int)Qt::Key_questiondown, 191},
        {(unsigned int)Qt::Key_Agrave, 192},
        {(unsigned int)Qt::Key_Aacute, 193},
        {(unsigned int)Qt::Key_Acircumflex, 194},
        {(unsigned int)Qt::Key_Atilde, 195},
        {(unsigned int)Qt::Key_Adiaeresis, 196},
        {(unsigned int)Qt::Key_Aring, 197},
        {(unsigned int)Qt::Key_AE, 198},
        {(unsigned int)Qt::Key_Ccedilla, 199},
        {(unsigned int)Qt::Key_Egrave, 200},
        {(unsigned int)Qt::Key_Eacute, 201},
        {(unsigned int)Qt::Key_Ecircumflex, 202},
        {(unsigned int)Qt::Key_Ediaeresis, 203},
        {(unsigned int)Qt::Key_Igrave, 204},
        {(unsigned int)Qt::Key_Iacute, 205},
        {(unsigned int)Qt::Key_Icircumflex, 206},
        {(unsigned int)Qt::Key_Idiaeresis, 207},
        {(unsigned int)Qt::Key_ETH, 208},
        {(unsigned int)Qt::Key_Ntilde, 209},
        {(unsigned int)Qt::Key_Ograve, 210},
        {(unsigned int)Qt::Key_Oacute, 211},
        {(unsigned int)Qt::Key_Ocircumflex, 212},
        {(unsigned int)Qt::Key_Otilde, 213},
        {(unsigned int)Qt::Key_Odiaeresis, 214},
        {(unsigned int)Qt::Key_multiply, 215},
        {(unsigned int)Qt::Key_Ooblique, 216},
        {(unsigned int)Qt::Key_Ugrave, 217},
        {(unsigned int)Qt::Key_Uacute, 218},
        {(unsigned int)Qt::Key_Ucircumflex, 219},
        {(unsigned int)Qt::Key_Udiaeresis, 220},
        {(unsigned int)Qt::Key_Yacute, 221},
        {(unsigned int)Qt::Key_THORN, 222},
        {(unsigned int)Qt::Key_ssharp, 223},
        {(unsigned int)Qt::Key_division, 247},
        {(unsigned int)Qt::Key_ydiaeresis, 255},
        {(unsigned int)Qt::Key_AltGr, 16781571},
        {(unsigned int)Qt::Key_Multi_key, 16781600},
        {(unsigned int)Qt::Key_Codeinput, 16781623},
        {(unsigned int)Qt::Key_SingleCandidate, 16781628},
        {(unsigned int)Qt::Key_MultipleCandidate, 16781629},
        {(unsigned int)Qt::Key_PreviousCandidate, 16781630},
        {(unsigned int)Qt::Key_Mode_switch, 16781694},
        {(unsigned int)Qt::Key_Kanji, 16781601},
        {(unsigned int)Qt::Key_Muhenkan, 16781602},
        {(unsigned int)Qt::Key_Henkan, 16781603},
        {(unsigned int)Qt::Key_Romaji, 16781604},
        {(unsigned int)Qt::Key_Hiragana, 16781605},
        {(unsigned int)Qt::Key_Katakana, 16781606},
        {(unsigned int)Qt::Key_Hiragana_Katakana, 16781607},
        {(unsigned int)Qt::Key_Zenkaku, 16781608},
        {(unsigned int)Qt::Key_Hankaku, 16781609},
        {(unsigned int)Qt::Key_Zenkaku_Hankaku, 16781610},
        {(unsigned int)Qt::Key_Touroku, 16781611},
        {(unsigned int)Qt::Key_Massyo, 16781612},
        {(unsigned int)Qt::Key_Kana_Lock, 16781613},
        {(unsigned int)Qt::Key_Kana_Shift, 16781614},
        {(unsigned int)Qt::Key_Eisu_Shift, 16781615},
        {(unsigned int)Qt::Key_Eisu_toggle, 16781616},
        {(unsigned int)Qt::Key_Hangul, 16781617},
        {(unsigned int)Qt::Key_Hangul_Start, 16781618},
        {(unsigned int)Qt::Key_Hangul_End, 16781619},
        {(unsigned int)Qt::Key_Hangul_Hanja, 16781620},
        {(unsigned int)Qt::Key_Hangul_Jamo, 16781621},
        {(unsigned int)Qt::Key_Hangul_Romaja, 16781622},
        {(unsigned int)Qt::Key_Hangul_Jeonja, 16781624},
        {(unsigned int)Qt::Key_Hangul_Banja, 16781625},
        {(unsigned int)Qt::Key_Hangul_PreHanja, 16781626},
        {(unsigned int)Qt::Key_Hangul_PostHanja, 16781627},
        {(unsigned int)Qt::Key_Hangul_Special, 16781631},
        {(unsigned int)Qt::Key_Dead_Grave, 16781904},
        {(unsigned int)Qt::Key_Dead_Acute, 16781905},
        {(unsigned int)Qt::Key_Dead_Circumflex, 16781906},
        {(unsigned int)Qt::Key_Dead_Tilde, 16781907},
        {(unsigned int)Qt::Key_Dead_Macron, 16781908},
        {(unsigned int)Qt::Key_Dead_Breve, 16781909},
        {(unsigned int)Qt::Key_Dead_Abovedot, 16781910},
        {(unsigned int)Qt::Key_Dead_Diaeresis, 16781911},
        {(unsigned int)Qt::Key_Dead_Abovering, 16781912},
        {(unsigned int)Qt::Key_Dead_Doubleacute, 16781913},
        {(unsigned int)Qt::Key_Dead_Caron, 16781914},
        {(unsigned int)Qt::Key_Dead_Cedilla, 16781915},
        {(unsigned int)Qt::Key_Dead_Ogonek, 16781916},
        {(unsigned int)Qt::Key_Dead_Iota, 16781917},
        {(unsigned int)Qt::Key_Dead_Voiced_Sound, 16781918},
        {(unsigned int)Qt::Key_Dead_Semivoiced_Sound, 16781919},
        {(unsigned int)Qt::Key_Dead_Belowdot, 16781920},
        {(unsigned int)Qt::Key_Dead_Hook, 16781921},
        {(unsigned int)Qt::Key_Dead_Horn, 16781922},
        {(unsigned int)Qt::Key_Dead_Stroke, 16781923},
        {(unsigned int)Qt::Key_Dead_Abovecomma, 16781924},
        {(unsigned int)Qt::Key_Dead_Abovereversedcomma, 16781925},
        {(unsigned int)Qt::Key_Dead_Doublegrave, 16781926},
        {(unsigned int)Qt::Key_Dead_Belowring, 16781927},
        {(unsigned int)Qt::Key_Dead_Belowmacron, 16781928},
        {(unsigned int)Qt::Key_Dead_Belowcircumflex, 16781929},
        {(unsigned int)Qt::Key_Dead_Belowtilde, 16781930},
        {(unsigned int)Qt::Key_Dead_Belowbreve, 16781931},
        {(unsigned int)Qt::Key_Dead_Belowdiaeresis, 16781932},
        {(unsigned int)Qt::Key_Dead_Invertedbreve, 16781933},
        {(unsigned int)Qt::Key_Dead_Belowcomma, 16781934},
        {(unsigned int)Qt::Key_Dead_Currency, 16781935},
        {(unsigned int)Qt::Key_Dead_a, 16781952},
        {(unsigned int)Qt::Key_Dead_A, 16781953},
        {(unsigned int)Qt::Key_Dead_e, 16781954},
        {(unsigned int)Qt::Key_Dead_E, 16781955},
        {(unsigned int)Qt::Key_Dead_i, 16781956},
        {(unsigned int)Qt::Key_Dead_I, 16781957},
        {(unsigned int)Qt::Key_Dead_o, 16781958},
        {(unsigned int)Qt::Key_Dead_O, 16781959},
        {(unsigned int)Qt::Key_Dead_u, 16781960},
        {(unsigned int)Qt::Key_Dead_U, 16781961},
        {(unsigned int)Qt::Key_Dead_Small_Schwa, 16781962},
        {(unsigned int)Qt::Key_Dead_Capital_Schwa, 16781963},
        {(unsigned int)Qt::Key_Dead_Greek, 16781964},
        {(unsigned int)Qt::Key_Dead_Lowline, 16781968},
        {(unsigned int)Qt::Key_Dead_Aboveverticalline, 16781969},
        {(unsigned int)Qt::Key_Dead_Belowverticalline, 16781970},
        {(unsigned int)Qt::Key_Dead_Longsolidusoverlay, 16781971},
        {(unsigned int)Qt::Key_Back, 16777313},
        {(unsigned int)Qt::Key_Forward, 16777314},
        {(unsigned int)Qt::Key_Stop, 16777315},
        {(unsigned int)Qt::Key_Refresh, 16777316},
        {(unsigned int)Qt::Key_VolumeDown, 16777328},
        {(unsigned int)Qt::Key_VolumeMute, 16777329},
        {(unsigned int)Qt::Key_VolumeUp, 16777330},
        {(unsigned int)Qt::Key_BassBoost, 16777331},
        {(unsigned int)Qt::Key_BassUp, 16777332},
        {(unsigned int)Qt::Key_BassDown, 16777333},
        {(unsigned int)Qt::Key_TrebleUp, 16777334},
        {(unsigned int)Qt::Key_TrebleDown, 16777335},
        {(unsigned int)Qt::Key_MediaPlay, 16777344},
        {(unsigned int)Qt::Key_MediaStop, 16777345},
        {(unsigned int)Qt::Key_MediaPrevious, 16777346},
        {(unsigned int)Qt::Key_MediaNext, 16777347},
        {(unsigned int)Qt::Key_MediaRecord, 16777348},
        {(unsigned int)Qt::Key_MediaPause, 16777349},
        {(unsigned int)Qt::Key_MediaTogglePlayPause, 16777350},
        {(unsigned int)Qt::Key_HomePage, 16777360},
        {(unsigned int)Qt::Key_Favorites, 16777361},
        {(unsigned int)Qt::Key_Search, 16777362},
        {(unsigned int)Qt::Key_Standby, 16777363},
        {(unsigned int)Qt::Key_OpenUrl, 16777364},
        {(unsigned int)Qt::Key_LaunchMail, 16777376},
        {(unsigned int)Qt::Key_LaunchMedia, 16777377},
        {(unsigned int)Qt::Key_Launch0, 16777378},
        {(unsigned int)Qt::Key_Launch1, 16777379},
        {(unsigned int)Qt::Key_Launch2, 16777380},
        {(unsigned int)Qt::Key_Launch3, 16777381},
        {(unsigned int)Qt::Key_Launch4, 16777382},
        {(unsigned int)Qt::Key_Launch5, 16777383},
        {(unsigned int)Qt::Key_Launch6, 16777384},
        {(unsigned int)Qt::Key_Launch7, 16777385},
        {(unsigned int)Qt::Key_Launch8, 16777386},
        {(unsigned int)Qt::Key_Launch9, 16777387},
        {(unsigned int)Qt::Key_LaunchA, 16777388},
        {(unsigned int)Qt::Key_LaunchB, 16777389},
        {(unsigned int)Qt::Key_LaunchC, 16777390},
        {(unsigned int)Qt::Key_LaunchD, 16777391},
        {(unsigned int)Qt::Key_LaunchE, 16777392},
        {(unsigned int)Qt::Key_LaunchF, 16777393},
        {(unsigned int)Qt::Key_MonBrightnessUp, 16777394},
        {(unsigned int)Qt::Key_MonBrightnessDown, 16777395},
        {(unsigned int)Qt::Key_KeyboardLightOnOff, 16777396},
        {(unsigned int)Qt::Key_KeyboardBrightnessUp, 16777397},
        {(unsigned int)Qt::Key_KeyboardBrightnessDown, 16777398},
        {(unsigned int)Qt::Key_PowerOff, 16777399},
        {(unsigned int)Qt::Key_WakeUp, 16777400},
        {(unsigned int)Qt::Key_Eject, 16777401},
        {(unsigned int)Qt::Key_ScreenSaver, 16777402},
        {(unsigned int)Qt::Key_WWW, 16777403},
        {(unsigned int)Qt::Key_Memo, 16777404},
        {(unsigned int)Qt::Key_LightBulb, 16777405},
        {(unsigned int)Qt::Key_Shop, 16777406},
        {(unsigned int)Qt::Key_History, 16777407},
        {(unsigned int)Qt::Key_AddFavorite, 16777408},
        {(unsigned int)Qt::Key_HotLinks, 16777409},
        {(unsigned int)Qt::Key_BrightnessAdjust, 16777410},
        {(unsigned int)Qt::Key_Finance, 16777411},
        {(unsigned int)Qt::Key_Community, 16777412},
        {(unsigned int)Qt::Key_AudioRewind, 16777413},
        {(unsigned int)Qt::Key_BackForward, 16777414},
        {(unsigned int)Qt::Key_ApplicationLeft, 16777415},
        {(unsigned int)Qt::Key_ApplicationRight, 16777416},
        {(unsigned int)Qt::Key_Book, 16777417},
        {(unsigned int)Qt::Key_CD, 16777418},
        {(unsigned int)Qt::Key_Calculator, 16777419},
        {(unsigned int)Qt::Key_ToDoList, 16777420},
        {(unsigned int)Qt::Key_ClearGrab, 16777421},
        {(unsigned int)Qt::Key_Close, 16777422},
        {(unsigned int)Qt::Key_Copy, 16777423},
        {(unsigned int)Qt::Key_Cut, 16777424},
        {(unsigned int)Qt::Key_Display, 16777425},
        {(unsigned int)Qt::Key_DOS, 16777426},
        {(unsigned int)Qt::Key_Documents, 16777427},
        {(unsigned int)Qt::Key_Excel, 16777428},
        {(unsigned int)Qt::Key_Explorer, 16777429},
        {(unsigned int)Qt::Key_Game, 16777430},
        {(unsigned int)Qt::Key_Go, 16777431},
        {(unsigned int)Qt::Key_iTouch, 16777432},
        {(unsigned int)Qt::Key_LogOff, 16777433},
        {(unsigned int)Qt::Key_Market, 16777434},
        {(unsigned int)Qt::Key_Meeting, 16777435},
        {(unsigned int)Qt::Key_MenuKB, 16777436},
        {(unsigned int)Qt::Key_MenuPB, 16777437},
        {(unsigned int)Qt::Key_MySites, 16777438},
        {(unsigned int)Qt::Key_News, 16777439},
        {(unsigned int)Qt::Key_OfficeHome, 16777440},
        {(unsigned int)Qt::Key_Option, 16777441},
        {(unsigned int)Qt::Key_Paste, 16777442},
        {(unsigned int)Qt::Key_Phone, 16777443},
        {(unsigned int)Qt::Key_Calendar, 16777444},
        {(unsigned int)Qt::Key_Reply, 16777445},
        {(unsigned int)Qt::Key_Reload, 16777446},
        {(unsigned int)Qt::Key_RotateWindows, 16777447},
        {(unsigned int)Qt::Key_RotationPB, 16777448},
        {(unsigned int)Qt::Key_RotationKB, 16777449},
        {(unsigned int)Qt::Key_Save, 16777450},
        {(unsigned int)Qt::Key_Send, 16777451},
        {(unsigned int)Qt::Key_Spell, 16777452},
        {(unsigned int)Qt::Key_SplitScreen, 16777453},
        {(unsigned int)Qt::Key_Support, 16777454},
        {(unsigned int)Qt::Key_TaskPane, 16777455},
        {(unsigned int)Qt::Key_Terminal, 16777456},
        {(unsigned int)Qt::Key_Tools, 16777457},
        {(unsigned int)Qt::Key_Travel, 16777458},
        {(unsigned int)Qt::Key_Video, 16777459},
        {(unsigned int)Qt::Key_Word, 16777460},
        {(unsigned int)Qt::Key_Xfer, 16777461},
        {(unsigned int)Qt::Key_ZoomIn, 16777462},
        {(unsigned int)Qt::Key_ZoomOut, 16777463},
        {(unsigned int)Qt::Key_Away, 16777464},
        {(unsigned int)Qt::Key_Messenger, 16777465},
        {(unsigned int)Qt::Key_WebCam, 16777466},
        {(unsigned int)Qt::Key_MailForward, 16777467},
        {(unsigned int)Qt::Key_Pictures, 16777468},
        {(unsigned int)Qt::Key_Music, 16777469},
        {(unsigned int)Qt::Key_Battery, 16777470},
        {(unsigned int)Qt::Key_Bluetooth, 16777471},
        {(unsigned int)Qt::Key_WLAN, 16777472},
        {(unsigned int)Qt::Key_UWB, 16777473},
        {(unsigned int)Qt::Key_AudioForward, 16777474},
        {(unsigned int)Qt::Key_AudioRepeat, 16777475},
        {(unsigned int)Qt::Key_AudioRandomPlay, 16777476},
        {(unsigned int)Qt::Key_Subtitle, 16777477},
        {(unsigned int)Qt::Key_AudioCycleTrack, 16777478},
        {(unsigned int)Qt::Key_Time, 16777479},
        {(unsigned int)Qt::Key_Hibernate, 16777480},
        {(unsigned int)Qt::Key_View, 16777481},
        {(unsigned int)Qt::Key_TopMenu, 16777482},
        {(unsigned int)Qt::Key_PowerDown, 16777483},
        {(unsigned int)Qt::Key_Suspend, 16777484},
        {(unsigned int)Qt::Key_ContrastAdjust, 16777485},
        {(unsigned int)Qt::Key_LaunchG, 16777486},
        {(unsigned int)Qt::Key_LaunchH, 16777487},
        {(unsigned int)Qt::Key_TouchpadToggle, 16777488},
        {(unsigned int)Qt::Key_TouchpadOn, 16777489},
        {(unsigned int)Qt::Key_TouchpadOff, 16777490},
        {(unsigned int)Qt::Key_MicMute, 16777491},
        {(unsigned int)Qt::Key_Red, 16777492},
        {(unsigned int)Qt::Key_Green, 16777493},
        {(unsigned int)Qt::Key_Yellow, 16777494},
        {(unsigned int)Qt::Key_Blue, 16777495},
        {(unsigned int)Qt::Key_ChannelUp, 16777496},
        {(unsigned int)Qt::Key_ChannelDown, 16777497},
        {(unsigned int)Qt::Key_Guide, 16777498},
        {(unsigned int)Qt::Key_Info, 16777499},
        {(unsigned int)Qt::Key_Settings, 16777500},
        {(unsigned int)Qt::Key_MicVolumeUp, 16777501},
        {(unsigned int)Qt::Key_MicVolumeDown, 16777502},
        {(unsigned int)Qt::Key_New, 16777504},
        {(unsigned int)Qt::Key_Open, 16777505},
        {(unsigned int)Qt::Key_Find, 16777506},
        {(unsigned int)Qt::Key_Undo, 16777507},
        {(unsigned int)Qt::Key_Redo, 16777508},
        {(unsigned int)Qt::Key_MediaLast, 16842751},
        {(unsigned int)Qt::Key_Select, 16842752},
        {(unsigned int)Qt::Key_Yes, 16842753},
        {(unsigned int)Qt::Key_No, 16842754},
        {(unsigned int)Qt::Key_Cancel, 16908289},
        {(unsigned int)Qt::Key_Printer, 16908290},
        {(unsigned int)Qt::Key_Execute, 16908291},
        {(unsigned int)Qt::Key_Sleep, 16908292},
        {(unsigned int)Qt::Key_Play, 16908293},
        {(unsigned int)Qt::Key_Zoom, 16908294},
        {(unsigned int)Qt::Key_Exit, 16908298},
        {(unsigned int)Qt::Key_Context1, 17825792},
        {(unsigned int)Qt::Key_Context2, 17825793},
        {(unsigned int)Qt::Key_Context3, 17825794},
        {(unsigned int)Qt::Key_Context4, 17825795},
        {(unsigned int)Qt::Key_Call, 17825796},
        {(unsigned int)Qt::Key_Hangup, 17825797},
        {(unsigned int)Qt::Key_Flip, 17825798},
        {(unsigned int)Qt::Key_ToggleCallHangup, 17825799},
        {(unsigned int)Qt::Key_VoiceDial, 17825800},
        {(unsigned int)Qt::Key_LastNumberRedial, 17825801},
        {(unsigned int)Qt::Key_Camera, 17825824},
        {(unsigned int)Qt::Key_CameraFocus, 17825825},
        {(unsigned int)Qt::Key_unknown, 33554431},
    };

    for (int i = 0; i < 469; ++i) {
        s_key_lookup[key_vals[i].key] = (unsigned int)key_vals[i].val;
    }

    static KeyVal keyboard_modifier_vals[] = {
        {(unsigned int)Qt::NoModifier, 0},
        {(unsigned int)Qt::ShiftModifier, 33554432},
        {(unsigned int)Qt::ControlModifier, 67108864},
        {(unsigned int)Qt::AltModifier, 134217728},
        {(unsigned int)Qt::MetaModifier, 268435456},
        {(unsigned int)Qt::KeypadModifier, 536870912},
        {(unsigned int)Qt::GroupSwitchModifier, 1073741824},
        {(unsigned int)Qt::KeyboardModifierMask, 4261412864},
    };

    for (int i = 0; i < 8; ++i) {
        s_keyboard_modifier_lookup[keyboard_modifier_vals[i].key] =
            (unsigned int)keyboard_modifier_vals[i].val;
    }

    static KeyVal layout_direction_vals[] = {
        {(unsigned int)Qt::LeftToRight, 0},
        {(unsigned int)Qt::RightToLeft, 1},
        {(unsigned int)Qt::LayoutDirectionAuto, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_layout_direction_lookup[layout_direction_vals[i].key] =
            (unsigned int)layout_direction_vals[i].val;
    }

    static KeyVal mask_mode_vals[] = {
        {(unsigned int)Qt::MaskInColor, 0},
        {(unsigned int)Qt::MaskOutColor, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_mask_mode_lookup[mask_mode_vals[i].key] =
            (unsigned int)mask_mode_vals[i].val;
    }

    static KeyVal match_flag_vals[] = {
        {(unsigned int)Qt::MatchExactly, 0},
        {(unsigned int)Qt::MatchContains, 1},
        {(unsigned int)Qt::MatchStartsWith, 2},
        {(unsigned int)Qt::MatchEndsWith, 3},
        {(unsigned int)Qt::MatchRegExp, 4},
        {(unsigned int)Qt::MatchWildcard, 5},
        {(unsigned int)Qt::MatchFixedString, 8},
        {(unsigned int)Qt::MatchCaseSensitive, 16},
        {(unsigned int)Qt::MatchWrap, 32},
        {(unsigned int)Qt::MatchRecursive, 64},
    };

    for (int i = 0; i < 10; ++i) {
        s_match_flag_lookup[match_flag_vals[i].key] =
            (unsigned int)match_flag_vals[i].val;
    }

    static KeyVal mode_vals[] = {
        {(unsigned int)QIcon::Normal, 0},
        {(unsigned int)QIcon::Disabled, 1},
        {(unsigned int)QIcon::Active, 2},
        {(unsigned int)QIcon::Selected, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_mode_lookup[mode_vals[i].key] = (unsigned int)mode_vals[i].val;
    }

    static KeyVal modifier_vals[] = {
        {(unsigned int)Qt::META, 268435456},
        {(unsigned int)Qt::SHIFT, 33554432},
        {(unsigned int)Qt::CTRL, 67108864},
        {(unsigned int)Qt::ALT, 134217728},
        {(unsigned int)Qt::MODIFIER_MASK, 4261412864},
        {(unsigned int)Qt::UNICODE_ACCEL, 0},
    };

    for (int i = 0; i < 6; ++i) {
        s_modifier_lookup[modifier_vals[i].key] =
            (unsigned int)modifier_vals[i].val;
    }

    static KeyVal mouse_button_vals[] = {
        {(unsigned int)Qt::NoButton, 0},
        {(unsigned int)Qt::LeftButton, 1},
        {(unsigned int)Qt::RightButton, 2},
        {(unsigned int)Qt::MidButton, 4},
        {(unsigned int)Qt::MiddleButton, 4},
        {(unsigned int)Qt::BackButton, 8},
        {(unsigned int)Qt::XButton1, 8},
        {(unsigned int)Qt::ExtraButton1, 8},
        {(unsigned int)Qt::ForwardButton, 16},
        {(unsigned int)Qt::XButton2, 16},
        {(unsigned int)Qt::ExtraButton2, 16},
        {(unsigned int)Qt::TaskButton, 32},
        {(unsigned int)Qt::ExtraButton3, 32},
        {(unsigned int)Qt::ExtraButton4, 64},
        {(unsigned int)Qt::ExtraButton5, 128},
        {(unsigned int)Qt::ExtraButton6, 256},
        {(unsigned int)Qt::ExtraButton7, 512},
        {(unsigned int)Qt::ExtraButton8, 1024},
        {(unsigned int)Qt::ExtraButton9, 2048},
        {(unsigned int)Qt::ExtraButton10, 4096},
        {(unsigned int)Qt::ExtraButton11, 8192},
        {(unsigned int)Qt::ExtraButton12, 16384},
        {(unsigned int)Qt::ExtraButton13, 32768},
        {(unsigned int)Qt::ExtraButton14, 65536},
        {(unsigned int)Qt::ExtraButton15, 131072},
        {(unsigned int)Qt::ExtraButton16, 262144},
        {(unsigned int)Qt::ExtraButton17, 524288},
        {(unsigned int)Qt::ExtraButton18, 1048576},
        {(unsigned int)Qt::ExtraButton19, 2097152},
        {(unsigned int)Qt::ExtraButton20, 4194304},
        {(unsigned int)Qt::ExtraButton21, 8388608},
        {(unsigned int)Qt::ExtraButton22, 16777216},
        {(unsigned int)Qt::ExtraButton23, 33554432},
        {(unsigned int)Qt::ExtraButton24, 67108864},
        {(unsigned int)Qt::AllButtons, 134217727},
        {(unsigned int)Qt::MaxMouseButton, 67108864},
        {(unsigned int)Qt::MouseButtonMask, 4294967295},
    };

    for (int i = 0; i < 37; ++i) {
        s_mouse_button_lookup[mouse_button_vals[i].key] =
            (unsigned int)mouse_button_vals[i].val;
    }

    static KeyVal mouse_event_flag_vals[] = {
        {(unsigned int)Qt::MouseEventCreatedDoubleClick, 1},
        {(unsigned int)Qt::MouseEventFlagMask, 255},
    };

    for (int i = 0; i < 2; ++i) {
        s_mouse_event_flag_lookup[mouse_event_flag_vals[i].key] =
            (unsigned int)mouse_event_flag_vals[i].val;
    }

    static KeyVal mouse_event_source_vals[] = {
        {(unsigned int)Qt::MouseEventNotSynthesized, 0},
        {(unsigned int)Qt::MouseEventSynthesizedBySystem, 1},
        {(unsigned int)Qt::MouseEventSynthesizedByQt, 2},
        {(unsigned int)Qt::MouseEventSynthesizedByApplication, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_mouse_event_source_lookup[mouse_event_source_vals[i].key] =
            (unsigned int)mouse_event_source_vals[i].val;
    }

    static KeyVal name_format_vals[] = {
        {(unsigned int)QColor::HexRgb, 0},
        {(unsigned int)QColor::HexArgb, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_name_format_lookup[name_format_vals[i].key] =
            (unsigned int)name_format_vals[i].val;
    }

    static KeyVal native_gesture_type_vals[] = {
        {(unsigned int)Qt::BeginNativeGesture, 0},
        {(unsigned int)Qt::EndNativeGesture, 1},
        {(unsigned int)Qt::PanNativeGesture, 2},
        {(unsigned int)Qt::ZoomNativeGesture, 3},
        {(unsigned int)Qt::SmartZoomNativeGesture, 4},
        {(unsigned int)Qt::RotateNativeGesture, 5},
        {(unsigned int)Qt::SwipeNativeGesture, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_native_gesture_type_lookup[native_gesture_type_vals[i].key] =
            (unsigned int)native_gesture_type_vals[i].val;
    }

    static KeyVal navigation_mode_vals[] = {
        {(unsigned int)Qt::NavigationModeNone, 0},
        {(unsigned int)Qt::NavigationModeKeypadTabOrder, 1},
        {(unsigned int)Qt::NavigationModeKeypadDirectional, 2},
        {(unsigned int)Qt::NavigationModeCursorAuto, 3},
        {(unsigned int)Qt::NavigationModeCursorForceVisible, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_navigation_mode_lookup[navigation_mode_vals[i].key] =
            (unsigned int)navigation_mode_vals[i].val;
    }

    static KeyVal open_gl_context_profile_vals[] = {
        {(unsigned int)QSurfaceFormat::NoProfile, 0},
        {(unsigned int)QSurfaceFormat::CoreProfile, 1},
        {(unsigned int)QSurfaceFormat::CompatibilityProfile, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_open_gl_context_profile_lookup[open_gl_context_profile_vals[i].key] =
            (unsigned int)open_gl_context_profile_vals[i].val;
    }

    static KeyVal orientation_vals[] = {
        {(unsigned int)Qt::Horizontal, 1},
        {(unsigned int)Qt::Vertical, 2},
    };

    for (int i = 0; i < 2; ++i) {
        s_orientation_lookup[orientation_vals[i].key] =
            (unsigned int)orientation_vals[i].val;
    }

    static KeyVal paint_device_metric_vals[] = {
        {(unsigned int)QPaintDevice::PdmWidth, 1},
        {(unsigned int)QPaintDevice::PdmHeight, 2},
        {(unsigned int)QPaintDevice::PdmWidthMM, 3},
        {(unsigned int)QPaintDevice::PdmHeightMM, 4},
        {(unsigned int)QPaintDevice::PdmNumColors, 5},
        {(unsigned int)QPaintDevice::PdmDepth, 6},
        {(unsigned int)QPaintDevice::PdmDpiX, 7},
        {(unsigned int)QPaintDevice::PdmDpiY, 8},
        {(unsigned int)QPaintDevice::PdmPhysicalDpiX, 9},
        {(unsigned int)QPaintDevice::PdmPhysicalDpiY, 10},
        {(unsigned int)QPaintDevice::PdmDevicePixelRatio, 11},
        {(unsigned int)QPaintDevice::PdmDevicePixelRatioScaled, 12},
    };

    for (int i = 0; i < 12; ++i) {
        s_paint_device_metric_lookup[paint_device_metric_vals[i].key] =
            (unsigned int)paint_device_metric_vals[i].val;
    }

    static KeyVal paint_engine_feature_vals[] = {
        {(unsigned int)QPaintEngine::PrimitiveTransform, 1},
        {(unsigned int)QPaintEngine::PatternTransform, 2},
        {(unsigned int)QPaintEngine::PixmapTransform, 4},
        {(unsigned int)QPaintEngine::PatternBrush, 8},
        {(unsigned int)QPaintEngine::LinearGradientFill, 16},
        {(unsigned int)QPaintEngine::RadialGradientFill, 32},
        {(unsigned int)QPaintEngine::ConicalGradientFill, 64},
        {(unsigned int)QPaintEngine::AlphaBlend, 128},
        {(unsigned int)QPaintEngine::PorterDuff, 256},
        {(unsigned int)QPaintEngine::PainterPaths, 512},
        {(unsigned int)QPaintEngine::Antialiasing, 1024},
        {(unsigned int)QPaintEngine::BrushStroke, 2048},
        {(unsigned int)QPaintEngine::ConstantOpacity, 4096},
        {(unsigned int)QPaintEngine::MaskedBrush, 8192},
        {(unsigned int)QPaintEngine::PerspectiveTransform, 16384},
        {(unsigned int)QPaintEngine::BlendModes, 32768},
        {(unsigned int)QPaintEngine::ObjectBoundingModeGradients, 65536},
        {(unsigned int)QPaintEngine::RasterOpModes, 131072},
        {(unsigned int)QPaintEngine::PaintOutsidePaintEvent, 536870912},
        {(unsigned int)QPaintEngine::AllFeatures, 4294967295},
    };

    for (int i = 0; i < 20; ++i) {
        s_paint_engine_feature_lookup[paint_engine_feature_vals[i].key] =
            (unsigned int)paint_engine_feature_vals[i].val;
    }

    static KeyVal pen_cap_style_vals[] = {
        {(unsigned int)Qt::FlatCap, 0},
        {(unsigned int)Qt::SquareCap, 16},
        {(unsigned int)Qt::RoundCap, 32},
        {(unsigned int)Qt::MPenCapStyle, 48},
    };

    for (int i = 0; i < 4; ++i) {
        s_pen_cap_style_lookup[pen_cap_style_vals[i].key] =
            (unsigned int)pen_cap_style_vals[i].val;
    }

    static KeyVal pen_join_style_vals[] = {
        {(unsigned int)Qt::MiterJoin, 0},
        {(unsigned int)Qt::BevelJoin, 64},
        {(unsigned int)Qt::RoundJoin, 128},
        {(unsigned int)Qt::SvgMiterJoin, 256},
        {(unsigned int)Qt::MPenJoinStyle, 448},
    };

    for (int i = 0; i < 5; ++i) {
        s_pen_join_style_lookup[pen_join_style_vals[i].key] =
            (unsigned int)pen_join_style_vals[i].val;
    }

    static KeyVal pen_style_vals[] = {
        {(unsigned int)Qt::NoPen, 0},
        {(unsigned int)Qt::SolidLine, 1},
        {(unsigned int)Qt::DashLine, 2},
        {(unsigned int)Qt::DotLine, 3},
        {(unsigned int)Qt::DashDotLine, 4},
        {(unsigned int)Qt::DashDotDotLine, 5},
        {(unsigned int)Qt::CustomDashLine, 6},
        {(unsigned int)Qt::MPenStyle, 15},
    };

    for (int i = 0; i < 8; ++i) {
        s_pen_style_lookup[pen_style_vals[i].key] =
            (unsigned int)pen_style_vals[i].val;
    }

    static KeyVal pixel_metric_vals[] = {
        {(unsigned int)QStyle::PM_ButtonMargin, 0},
        {(unsigned int)QStyle::PM_ButtonDefaultIndicator, 1},
        {(unsigned int)QStyle::PM_MenuButtonIndicator, 2},
        {(unsigned int)QStyle::PM_ButtonShiftHorizontal, 3},
        {(unsigned int)QStyle::PM_ButtonShiftVertical, 4},
        {(unsigned int)QStyle::PM_DefaultFrameWidth, 5},
        {(unsigned int)QStyle::PM_SpinBoxFrameWidth, 6},
        {(unsigned int)QStyle::PM_ComboBoxFrameWidth, 7},
        {(unsigned int)QStyle::PM_MaximumDragDistance, 8},
        {(unsigned int)QStyle::PM_ScrollBarExtent, 9},
        {(unsigned int)QStyle::PM_ScrollBarSliderMin, 10},
        {(unsigned int)QStyle::PM_SliderThickness, 11},
        {(unsigned int)QStyle::PM_SliderControlThickness, 12},
        {(unsigned int)QStyle::PM_SliderLength, 13},
        {(unsigned int)QStyle::PM_SliderTickmarkOffset, 14},
        {(unsigned int)QStyle::PM_SliderSpaceAvailable, 15},
        {(unsigned int)QStyle::PM_DockWidgetSeparatorExtent, 16},
        {(unsigned int)QStyle::PM_DockWidgetHandleExtent, 17},
        {(unsigned int)QStyle::PM_DockWidgetFrameWidth, 18},
        {(unsigned int)QStyle::PM_TabBarTabOverlap, 19},
        {(unsigned int)QStyle::PM_TabBarTabHSpace, 20},
        {(unsigned int)QStyle::PM_TabBarTabVSpace, 21},
        {(unsigned int)QStyle::PM_TabBarBaseHeight, 22},
        {(unsigned int)QStyle::PM_TabBarBaseOverlap, 23},
        {(unsigned int)QStyle::PM_ProgressBarChunkWidth, 24},
        {(unsigned int)QStyle::PM_SplitterWidth, 25},
        {(unsigned int)QStyle::PM_TitleBarHeight, 26},
        {(unsigned int)QStyle::PM_MenuScrollerHeight, 27},
        {(unsigned int)QStyle::PM_MenuHMargin, 28},
        {(unsigned int)QStyle::PM_MenuVMargin, 29},
        {(unsigned int)QStyle::PM_MenuPanelWidth, 30},
        {(unsigned int)QStyle::PM_MenuTearoffHeight, 31},
        {(unsigned int)QStyle::PM_MenuDesktopFrameWidth, 32},
        {(unsigned int)QStyle::PM_MenuBarPanelWidth, 33},
        {(unsigned int)QStyle::PM_MenuBarItemSpacing, 34},
        {(unsigned int)QStyle::PM_MenuBarVMargin, 35},
        {(unsigned int)QStyle::PM_MenuBarHMargin, 36},
        {(unsigned int)QStyle::PM_IndicatorWidth, 37},
        {(unsigned int)QStyle::PM_IndicatorHeight, 38},
        {(unsigned int)QStyle::PM_ExclusiveIndicatorWidth, 39},
        {(unsigned int)QStyle::PM_ExclusiveIndicatorHeight, 40},
        {(unsigned int)QStyle::PM_DialogButtonsSeparator, 41},
        {(unsigned int)QStyle::PM_DialogButtonsButtonWidth, 42},
        {(unsigned int)QStyle::PM_DialogButtonsButtonHeight, 43},
        {(unsigned int)QStyle::PM_MdiSubWindowFrameWidth, 44},
        {(unsigned int)QStyle::PM_MDIFrameWidth, 44},
        {(unsigned int)QStyle::PM_MdiSubWindowMinimizedWidth, 45},
        {(unsigned int)QStyle::PM_MDIMinimizedWidth, 45},
        {(unsigned int)QStyle::PM_HeaderMargin, 46},
        {(unsigned int)QStyle::PM_HeaderMarkSize, 47},
        {(unsigned int)QStyle::PM_HeaderGripMargin, 48},
        {(unsigned int)QStyle::PM_TabBarTabShiftHorizontal, 49},
        {(unsigned int)QStyle::PM_TabBarTabShiftVertical, 50},
        {(unsigned int)QStyle::PM_TabBarScrollButtonWidth, 51},
        {(unsigned int)QStyle::PM_ToolBarFrameWidth, 52},
        {(unsigned int)QStyle::PM_ToolBarHandleExtent, 53},
        {(unsigned int)QStyle::PM_ToolBarItemSpacing, 54},
        {(unsigned int)QStyle::PM_ToolBarItemMargin, 55},
        {(unsigned int)QStyle::PM_ToolBarSeparatorExtent, 56},
        {(unsigned int)QStyle::PM_ToolBarExtensionExtent, 57},
        {(unsigned int)QStyle::PM_SpinBoxSliderHeight, 58},
        {(unsigned int)QStyle::PM_DefaultTopLevelMargin, 59},
        {(unsigned int)QStyle::PM_DefaultChildMargin, 60},
        {(unsigned int)QStyle::PM_DefaultLayoutSpacing, 61},
        {(unsigned int)QStyle::PM_ToolBarIconSize, 62},
        {(unsigned int)QStyle::PM_ListViewIconSize, 63},
        {(unsigned int)QStyle::PM_IconViewIconSize, 64},
        {(unsigned int)QStyle::PM_SmallIconSize, 65},
        {(unsigned int)QStyle::PM_LargeIconSize, 66},
        {(unsigned int)QStyle::PM_FocusFrameVMargin, 67},
        {(unsigned int)QStyle::PM_FocusFrameHMargin, 68},
        {(unsigned int)QStyle::PM_ToolTipLabelFrameWidth, 69},
        {(unsigned int)QStyle::PM_CheckBoxLabelSpacing, 70},
        {(unsigned int)QStyle::PM_TabBarIconSize, 71},
        {(unsigned int)QStyle::PM_SizeGripSize, 72},
        {(unsigned int)QStyle::PM_DockWidgetTitleMargin, 73},
        {(unsigned int)QStyle::PM_MessageBoxIconSize, 74},
        {(unsigned int)QStyle::PM_ButtonIconSize, 75},
        {(unsigned int)QStyle::PM_DockWidgetTitleBarButtonMargin, 76},
        {(unsigned int)QStyle::PM_RadioButtonLabelSpacing, 77},
        {(unsigned int)QStyle::PM_LayoutLeftMargin, 78},
        {(unsigned int)QStyle::PM_LayoutTopMargin, 79},
        {(unsigned int)QStyle::PM_LayoutRightMargin, 80},
        {(unsigned int)QStyle::PM_LayoutBottomMargin, 81},
        {(unsigned int)QStyle::PM_LayoutHorizontalSpacing, 82},
        {(unsigned int)QStyle::PM_LayoutVerticalSpacing, 83},
        {(unsigned int)QStyle::PM_TabBar_ScrollButtonOverlap, 84},
        {(unsigned int)QStyle::PM_TextCursorWidth, 85},
        {(unsigned int)QStyle::PM_TabCloseIndicatorWidth, 86},
        {(unsigned int)QStyle::PM_TabCloseIndicatorHeight, 87},
        {(unsigned int)QStyle::PM_ScrollView_ScrollBarSpacing, 88},
        {(unsigned int)QStyle::PM_ScrollView_ScrollBarOverlap, 89},
        {(unsigned int)QStyle::PM_SubMenuOverlap, 90},
        {(unsigned int)QStyle::PM_TreeViewIndentation, 91},
        {(unsigned int)QStyle::PM_HeaderDefaultSectionSizeHorizontal, 92},
        {(unsigned int)QStyle::PM_HeaderDefaultSectionSizeVertical, 93},
        {(unsigned int)QStyle::PM_TitleBarButtonIconSize, 94},
        {(unsigned int)QStyle::PM_TitleBarButtonSize, 95},
        {(unsigned int)QStyle::PM_CustomBase, 4026531840},
    };

    for (int i = 0; i < 99; ++i) {
        s_pixel_metric_lookup[pixel_metric_vals[i].key] =
            (unsigned int)pixel_metric_vals[i].val;
    }

    static KeyVal pixmap_fragment_hint_vals[] = {
        {(unsigned int)QPainter::OpaqueHint, 1},
    };

    for (int i = 0; i < 1; ++i) {
        s_pixmap_fragment_hint_lookup[pixmap_fragment_hint_vals[i].key] =
            (unsigned int)pixmap_fragment_hint_vals[i].val;
    }

    static KeyVal pointer_type_vals[] = {
        {(unsigned int)QTabletEvent::UnknownPointer, 0},
        {(unsigned int)QTabletEvent::Pen, 1},
        {(unsigned int)QTabletEvent::Cursor, 2},
        {(unsigned int)QTabletEvent::Eraser, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_pointer_type_lookup[pointer_type_vals[i].key] =
            (unsigned int)pointer_type_vals[i].val;
    }

    static KeyVal policy_vals[] = {
        {(unsigned int)QSizePolicy::Fixed, 0},
        {(unsigned int)QSizePolicy::Minimum, 1},
        {(unsigned int)QSizePolicy::Maximum, 4},
        {(unsigned int)QSizePolicy::Preferred, 5},
        {(unsigned int)QSizePolicy::MinimumExpanding, 3},
        {(unsigned int)QSizePolicy::Expanding, 7},
        {(unsigned int)QSizePolicy::Ignored, 13},
    };

    for (int i = 0; i < 7; ++i) {
        s_policy_lookup[policy_vals[i].key] = (unsigned int)policy_vals[i].val;
    }

    static KeyVal policy_flag_vals[] = {
        {(unsigned int)QSizePolicy::GrowFlag, 1},
        {(unsigned int)QSizePolicy::ExpandFlag, 2},
        {(unsigned int)QSizePolicy::ShrinkFlag, 4},
        {(unsigned int)QSizePolicy::IgnoreFlag, 8},
    };

    for (int i = 0; i < 4; ++i) {
        s_policy_flag_lookup[policy_flag_vals[i].key] =
            (unsigned int)policy_flag_vals[i].val;
    }

    static KeyVal polygon_draw_mode_vals[] = {
        {(unsigned int)QPaintEngine::OddEvenMode, 0},
        {(unsigned int)QPaintEngine::WindingMode, 1},
        {(unsigned int)QPaintEngine::ConvexMode, 2},
        {(unsigned int)QPaintEngine::PolylineMode, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_polygon_draw_mode_lookup[polygon_draw_mode_vals[i].key] =
            (unsigned int)polygon_draw_mode_vals[i].val;
    }

    static KeyVal primitive_element_vals[] = {
        {(unsigned int)QStyle::PE_Frame, 0},
        {(unsigned int)QStyle::PE_FrameDefaultButton, 1},
        {(unsigned int)QStyle::PE_FrameDockWidget, 2},
        {(unsigned int)QStyle::PE_FrameFocusRect, 3},
        {(unsigned int)QStyle::PE_FrameGroupBox, 4},
        {(unsigned int)QStyle::PE_FrameLineEdit, 5},
        {(unsigned int)QStyle::PE_FrameMenu, 6},
        {(unsigned int)QStyle::PE_FrameStatusBar, 7},
        {(unsigned int)QStyle::PE_FrameStatusBarItem, 7},
        {(unsigned int)QStyle::PE_FrameTabWidget, 8},
        {(unsigned int)QStyle::PE_FrameWindow, 9},
        {(unsigned int)QStyle::PE_FrameButtonBevel, 10},
        {(unsigned int)QStyle::PE_FrameButtonTool, 11},
        {(unsigned int)QStyle::PE_FrameTabBarBase, 12},
        {(unsigned int)QStyle::PE_PanelButtonCommand, 13},
        {(unsigned int)QStyle::PE_PanelButtonBevel, 14},
        {(unsigned int)QStyle::PE_PanelButtonTool, 15},
        {(unsigned int)QStyle::PE_PanelMenuBar, 16},
        {(unsigned int)QStyle::PE_PanelToolBar, 17},
        {(unsigned int)QStyle::PE_PanelLineEdit, 18},
        {(unsigned int)QStyle::PE_IndicatorArrowDown, 19},
        {(unsigned int)QStyle::PE_IndicatorArrowLeft, 20},
        {(unsigned int)QStyle::PE_IndicatorArrowRight, 21},
        {(unsigned int)QStyle::PE_IndicatorArrowUp, 22},
        {(unsigned int)QStyle::PE_IndicatorBranch, 23},
        {(unsigned int)QStyle::PE_IndicatorButtonDropDown, 24},
        {(unsigned int)QStyle::PE_IndicatorViewItemCheck, 25},
        {(unsigned int)QStyle::PE_IndicatorItemViewItemCheck, 25},
        {(unsigned int)QStyle::PE_IndicatorCheckBox, 26},
        {(unsigned int)QStyle::PE_IndicatorDockWidgetResizeHandle, 27},
        {(unsigned int)QStyle::PE_IndicatorHeaderArrow, 28},
        {(unsigned int)QStyle::PE_IndicatorMenuCheckMark, 29},
        {(unsigned int)QStyle::PE_IndicatorProgressChunk, 30},
        {(unsigned int)QStyle::PE_IndicatorRadioButton, 31},
        {(unsigned int)QStyle::PE_IndicatorSpinDown, 32},
        {(unsigned int)QStyle::PE_IndicatorSpinMinus, 33},
        {(unsigned int)QStyle::PE_IndicatorSpinPlus, 34},
        {(unsigned int)QStyle::PE_IndicatorSpinUp, 35},
        {(unsigned int)QStyle::PE_IndicatorToolBarHandle, 36},
        {(unsigned int)QStyle::PE_IndicatorToolBarSeparator, 37},
        {(unsigned int)QStyle::PE_PanelTipLabel, 38},
        {(unsigned int)QStyle::PE_IndicatorTabTear, 39},
        {(unsigned int)QStyle::PE_IndicatorTabTearLeft, 39},
        {(unsigned int)QStyle::PE_PanelScrollAreaCorner, 40},
        {(unsigned int)QStyle::PE_Widget, 41},
        {(unsigned int)QStyle::PE_IndicatorColumnViewArrow, 42},
        {(unsigned int)QStyle::PE_IndicatorItemViewItemDrop, 43},
        {(unsigned int)QStyle::PE_PanelItemViewItem, 44},
        {(unsigned int)QStyle::PE_PanelItemViewRow, 45},
        {(unsigned int)QStyle::PE_PanelStatusBar, 46},
        {(unsigned int)QStyle::PE_IndicatorTabClose, 47},
        {(unsigned int)QStyle::PE_PanelMenu, 48},
        {(unsigned int)QStyle::PE_IndicatorTabTearRight, 49},
        {(unsigned int)QStyle::PE_CustomBase, 251658240},
    };

    for (int i = 0; i < 54; ++i) {
        s_primitive_element_lookup[primitive_element_vals[i].key] =
            (unsigned int)primitive_element_vals[i].val;
    }

    static KeyVal reason_vals[] = {
        {(unsigned int)QContextMenuEvent::Mouse, 0},
        {(unsigned int)QContextMenuEvent::Keyboard, 1},
        {(unsigned int)QContextMenuEvent::Other, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_reason_lookup[reason_vals[i].key] = (unsigned int)reason_vals[i].val;
    }

    static KeyVal region_type_vals[] = {
        {(unsigned int)QRegion::Rectangle, 0},
        {(unsigned int)QRegion::Ellipse, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_region_type_lookup[region_type_vals[i].key] =
            (unsigned int)region_type_vals[i].val;
    }

    static KeyVal render_flag_vals[] = {
        {(unsigned int)QWidget::DrawWindowBackground, 1},
        {(unsigned int)QWidget::DrawChildren, 2},
        {(unsigned int)QWidget::IgnoreMask, 4},
    };

    for (int i = 0; i < 3; ++i) {
        s_render_flag_lookup[render_flag_vals[i].key] =
            (unsigned int)render_flag_vals[i].val;
    }

    static KeyVal render_hint_vals[] = {
        {(unsigned int)QPainter::Antialiasing, 1},
        {(unsigned int)QPainter::TextAntialiasing, 2},
        {(unsigned int)QPainter::SmoothPixmapTransform, 4},
        {(unsigned int)QPainter::HighQualityAntialiasing, 8},
        {(unsigned int)QPainter::NonCosmeticDefaultPen, 16},
        {(unsigned int)QPainter::Qt4CompatiblePainting, 32},
    };

    for (int i = 0; i < 6; ++i) {
        s_render_hint_lookup[render_hint_vals[i].key] =
            (unsigned int)render_hint_vals[i].val;
    }

    static KeyVal renderable_type_vals[] = {
        {(unsigned int)QSurfaceFormat::DefaultRenderableType, 0},
        {(unsigned int)QSurfaceFormat::OpenGL, 1},
        {(unsigned int)QSurfaceFormat::OpenGLES, 2},
        {(unsigned int)QSurfaceFormat::OpenVG, 4},
    };

    for (int i = 0; i < 4; ++i) {
        s_renderable_type_lookup[renderable_type_vals[i].key] =
            (unsigned int)renderable_type_vals[i].val;
    }

    static KeyVal request_software_input_panel_vals[] = {
        {(unsigned int)QStyle::RSIP_OnMouseClickAndAlreadyFocused, 0},
        {(unsigned int)QStyle::RSIP_OnMouseClick, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_request_software_input_panel_lookup
            [request_software_input_panel_vals[i].key] =
                (unsigned int)request_software_input_panel_vals[i].val;
    }

    static KeyVal screen_orientation_vals[] = {
        {(unsigned int)Qt::PrimaryOrientation, 0},
        {(unsigned int)Qt::PortraitOrientation, 1},
        {(unsigned int)Qt::LandscapeOrientation, 2},
        {(unsigned int)Qt::InvertedPortraitOrientation, 4},
        {(unsigned int)Qt::InvertedLandscapeOrientation, 8},
    };

    for (int i = 0; i < 5; ++i) {
        s_screen_orientation_lookup[screen_orientation_vals[i].key] =
            (unsigned int)screen_orientation_vals[i].val;
    }

    static KeyVal scroll_bar_policy_vals[] = {
        {(unsigned int)Qt::ScrollBarAsNeeded, 0},
        {(unsigned int)Qt::ScrollBarAlwaysOff, 1},
        {(unsigned int)Qt::ScrollBarAlwaysOn, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_scroll_bar_policy_lookup[scroll_bar_policy_vals[i].key] =
            (unsigned int)scroll_bar_policy_vals[i].val;
    }

    static KeyVal scroll_phase_vals[] = {
        {(unsigned int)Qt::NoScrollPhase, 0},
        {(unsigned int)Qt::ScrollBegin, 1},
        {(unsigned int)Qt::ScrollUpdate, 2},
        {(unsigned int)Qt::ScrollEnd, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_scroll_phase_lookup[scroll_phase_vals[i].key] =
            (unsigned int)scroll_phase_vals[i].val;
    }

    static KeyVal sequence_format_vals[] = {
        {(unsigned int)QKeySequence::NativeText, 0},
        {(unsigned int)QKeySequence::PortableText, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_sequence_format_lookup[sequence_format_vals[i].key] =
            (unsigned int)sequence_format_vals[i].val;
    }

    static KeyVal sequence_match_vals[] = {
        {(unsigned int)QKeySequence::NoMatch, 0},
        {(unsigned int)QKeySequence::PartialMatch, 1},
        {(unsigned int)QKeySequence::ExactMatch, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_sequence_match_lookup[sequence_match_vals[i].key] =
            (unsigned int)sequence_match_vals[i].val;
    }

    static KeyVal shortcut_context_vals[] = {
        {(unsigned int)Qt::WidgetShortcut, 0},
        {(unsigned int)Qt::WindowShortcut, 1},
        {(unsigned int)Qt::ApplicationShortcut, 2},
        {(unsigned int)Qt::WidgetWithChildrenShortcut, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_shortcut_context_lookup[shortcut_context_vals[i].key] =
            (unsigned int)shortcut_context_vals[i].val;
    }

    static KeyVal size_constraint_vals[] = {
        {(unsigned int)QLayout::SetDefaultConstraint, 0},
        {(unsigned int)QLayout::SetNoConstraint, 1},
        {(unsigned int)QLayout::SetMinimumSize, 2},
        {(unsigned int)QLayout::SetFixedSize, 3},
        {(unsigned int)QLayout::SetMaximumSize, 4},
        {(unsigned int)QLayout::SetMinAndMaxSize, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_size_constraint_lookup[size_constraint_vals[i].key] =
            (unsigned int)size_constraint_vals[i].val;
    }

    static KeyVal size_hint_vals[] = {
        {(unsigned int)Qt::MinimumSize, 0},
        {(unsigned int)Qt::PreferredSize, 1},
        {(unsigned int)Qt::MaximumSize, 2},
        {(unsigned int)Qt::MinimumDescent, 3},
        {(unsigned int)Qt::NSizeHints, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_size_hint_lookup[size_hint_vals[i].key] =
            (unsigned int)size_hint_vals[i].val;
    }

    static KeyVal size_mode_vals[] = {
        {(unsigned int)Qt::AbsoluteSize, 0},
        {(unsigned int)Qt::RelativeSize, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_size_mode_lookup[size_mode_vals[i].key] =
            (unsigned int)size_mode_vals[i].val;
    }

    static KeyVal sort_order_vals[] = {
        {(unsigned int)Qt::AscendingOrder, 0},
        {(unsigned int)Qt::DescendingOrder, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_sort_order_lookup[sort_order_vals[i].key] =
            (unsigned int)sort_order_vals[i].val;
    }

    static KeyVal spacing_type_vals[] = {
        {(unsigned int)QFont::PercentageSpacing, 0},
        {(unsigned int)QFont::AbsoluteSpacing, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_spacing_type_lookup[spacing_type_vals[i].key] =
            (unsigned int)spacing_type_vals[i].val;
    }

    static KeyVal spec_vals[] = {
        {(unsigned int)QColor::Invalid, 0}, {(unsigned int)QColor::Rgb, 1},
        {(unsigned int)QColor::Hsv, 2},     {(unsigned int)QColor::Cmyk, 3},
        {(unsigned int)QColor::Hsl, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_spec_lookup[spec_vals[i].key] = (unsigned int)spec_vals[i].val;
    }

    static KeyVal spread_vals[] = {
        {(unsigned int)QGradient::PadSpread, 0},
        {(unsigned int)QGradient::ReflectSpread, 1},
        {(unsigned int)QGradient::RepeatSpread, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_spread_lookup[spread_vals[i].key] = (unsigned int)spread_vals[i].val;
    }

    static KeyVal standard_key_vals[] = {
        {(unsigned int)QKeySequence::UnknownKey, 0},
        {(unsigned int)QKeySequence::HelpContents, 1},
        {(unsigned int)QKeySequence::WhatsThis, 2},
        {(unsigned int)QKeySequence::Open, 3},
        {(unsigned int)QKeySequence::Close, 4},
        {(unsigned int)QKeySequence::Save, 5},
        {(unsigned int)QKeySequence::New, 6},
        {(unsigned int)QKeySequence::Delete, 7},
        {(unsigned int)QKeySequence::Cut, 8},
        {(unsigned int)QKeySequence::Copy, 9},
        {(unsigned int)QKeySequence::Paste, 10},
        {(unsigned int)QKeySequence::Undo, 11},
        {(unsigned int)QKeySequence::Redo, 12},
        {(unsigned int)QKeySequence::Back, 13},
        {(unsigned int)QKeySequence::Forward, 14},
        {(unsigned int)QKeySequence::Refresh, 15},
        {(unsigned int)QKeySequence::ZoomIn, 16},
        {(unsigned int)QKeySequence::ZoomOut, 17},
        {(unsigned int)QKeySequence::Print, 18},
        {(unsigned int)QKeySequence::AddTab, 19},
        {(unsigned int)QKeySequence::NextChild, 20},
        {(unsigned int)QKeySequence::PreviousChild, 21},
        {(unsigned int)QKeySequence::Find, 22},
        {(unsigned int)QKeySequence::FindNext, 23},
        {(unsigned int)QKeySequence::FindPrevious, 24},
        {(unsigned int)QKeySequence::Replace, 25},
        {(unsigned int)QKeySequence::SelectAll, 26},
        {(unsigned int)QKeySequence::Bold, 27},
        {(unsigned int)QKeySequence::Italic, 28},
        {(unsigned int)QKeySequence::Underline, 29},
        {(unsigned int)QKeySequence::MoveToNextChar, 30},
        {(unsigned int)QKeySequence::MoveToPreviousChar, 31},
        {(unsigned int)QKeySequence::MoveToNextWord, 32},
        {(unsigned int)QKeySequence::MoveToPreviousWord, 33},
        {(unsigned int)QKeySequence::MoveToNextLine, 34},
        {(unsigned int)QKeySequence::MoveToPreviousLine, 35},
        {(unsigned int)QKeySequence::MoveToNextPage, 36},
        {(unsigned int)QKeySequence::MoveToPreviousPage, 37},
        {(unsigned int)QKeySequence::MoveToStartOfLine, 38},
        {(unsigned int)QKeySequence::MoveToEndOfLine, 39},
        {(unsigned int)QKeySequence::MoveToStartOfBlock, 40},
        {(unsigned int)QKeySequence::MoveToEndOfBlock, 41},
        {(unsigned int)QKeySequence::MoveToStartOfDocument, 42},
        {(unsigned int)QKeySequence::MoveToEndOfDocument, 43},
        {(unsigned int)QKeySequence::SelectNextChar, 44},
        {(unsigned int)QKeySequence::SelectPreviousChar, 45},
        {(unsigned int)QKeySequence::SelectNextWord, 46},
        {(unsigned int)QKeySequence::SelectPreviousWord, 47},
        {(unsigned int)QKeySequence::SelectNextLine, 48},
        {(unsigned int)QKeySequence::SelectPreviousLine, 49},
        {(unsigned int)QKeySequence::SelectNextPage, 50},
        {(unsigned int)QKeySequence::SelectPreviousPage, 51},
        {(unsigned int)QKeySequence::SelectStartOfLine, 52},
        {(unsigned int)QKeySequence::SelectEndOfLine, 53},
        {(unsigned int)QKeySequence::SelectStartOfBlock, 54},
        {(unsigned int)QKeySequence::SelectEndOfBlock, 55},
        {(unsigned int)QKeySequence::SelectStartOfDocument, 56},
        {(unsigned int)QKeySequence::SelectEndOfDocument, 57},
        {(unsigned int)QKeySequence::DeleteStartOfWord, 58},
        {(unsigned int)QKeySequence::DeleteEndOfWord, 59},
        {(unsigned int)QKeySequence::DeleteEndOfLine, 60},
        {(unsigned int)QKeySequence::InsertParagraphSeparator, 61},
        {(unsigned int)QKeySequence::InsertLineSeparator, 62},
        {(unsigned int)QKeySequence::SaveAs, 63},
        {(unsigned int)QKeySequence::Preferences, 64},
        {(unsigned int)QKeySequence::Quit, 65},
        {(unsigned int)QKeySequence::FullScreen, 66},
        {(unsigned int)QKeySequence::Deselect, 67},
        {(unsigned int)QKeySequence::DeleteCompleteLine, 68},
        {(unsigned int)QKeySequence::Backspace, 69},
        {(unsigned int)QKeySequence::Cancel, 70},
    };

    for (int i = 0; i < 71; ++i) {
        s_standard_key_lookup[standard_key_vals[i].key] =
            (unsigned int)standard_key_vals[i].val;
    }

    static KeyVal standard_pixmap_vals[] = {
        {(unsigned int)QStyle::SP_TitleBarMenuButton, 0},
        {(unsigned int)QStyle::SP_TitleBarMinButton, 1},
        {(unsigned int)QStyle::SP_TitleBarMaxButton, 2},
        {(unsigned int)QStyle::SP_TitleBarCloseButton, 3},
        {(unsigned int)QStyle::SP_TitleBarNormalButton, 4},
        {(unsigned int)QStyle::SP_TitleBarShadeButton, 5},
        {(unsigned int)QStyle::SP_TitleBarUnshadeButton, 6},
        {(unsigned int)QStyle::SP_TitleBarContextHelpButton, 7},
        {(unsigned int)QStyle::SP_DockWidgetCloseButton, 8},
        {(unsigned int)QStyle::SP_MessageBoxInformation, 9},
        {(unsigned int)QStyle::SP_MessageBoxWarning, 10},
        {(unsigned int)QStyle::SP_MessageBoxCritical, 11},
        {(unsigned int)QStyle::SP_MessageBoxQuestion, 12},
        {(unsigned int)QStyle::SP_DesktopIcon, 13},
        {(unsigned int)QStyle::SP_TrashIcon, 14},
        {(unsigned int)QStyle::SP_ComputerIcon, 15},
        {(unsigned int)QStyle::SP_DriveFDIcon, 16},
        {(unsigned int)QStyle::SP_DriveHDIcon, 17},
        {(unsigned int)QStyle::SP_DriveCDIcon, 18},
        {(unsigned int)QStyle::SP_DriveDVDIcon, 19},
        {(unsigned int)QStyle::SP_DriveNetIcon, 20},
        {(unsigned int)QStyle::SP_DirOpenIcon, 21},
        {(unsigned int)QStyle::SP_DirClosedIcon, 22},
        {(unsigned int)QStyle::SP_DirLinkIcon, 23},
        {(unsigned int)QStyle::SP_DirLinkOpenIcon, 24},
        {(unsigned int)QStyle::SP_FileIcon, 25},
        {(unsigned int)QStyle::SP_FileLinkIcon, 26},
        {(unsigned int)QStyle::SP_ToolBarHorizontalExtensionButton, 27},
        {(unsigned int)QStyle::SP_ToolBarVerticalExtensionButton, 28},
        {(unsigned int)QStyle::SP_FileDialogStart, 29},
        {(unsigned int)QStyle::SP_FileDialogEnd, 30},
        {(unsigned int)QStyle::SP_FileDialogToParent, 31},
        {(unsigned int)QStyle::SP_FileDialogNewFolder, 32},
        {(unsigned int)QStyle::SP_FileDialogDetailedView, 33},
        {(unsigned int)QStyle::SP_FileDialogInfoView, 34},
        {(unsigned int)QStyle::SP_FileDialogContentsView, 35},
        {(unsigned int)QStyle::SP_FileDialogListView, 36},
        {(unsigned int)QStyle::SP_FileDialogBack, 37},
        {(unsigned int)QStyle::SP_DirIcon, 38},
        {(unsigned int)QStyle::SP_DialogOkButton, 39},
        {(unsigned int)QStyle::SP_DialogCancelButton, 40},
        {(unsigned int)QStyle::SP_DialogHelpButton, 41},
        {(unsigned int)QStyle::SP_DialogOpenButton, 42},
        {(unsigned int)QStyle::SP_DialogSaveButton, 43},
        {(unsigned int)QStyle::SP_DialogCloseButton, 44},
        {(unsigned int)QStyle::SP_DialogApplyButton, 45},
        {(unsigned int)QStyle::SP_DialogResetButton, 46},
        {(unsigned int)QStyle::SP_DialogDiscardButton, 47},
        {(unsigned int)QStyle::SP_DialogYesButton, 48},
        {(unsigned int)QStyle::SP_DialogNoButton, 49},
        {(unsigned int)QStyle::SP_ArrowUp, 50},
        {(unsigned int)QStyle::SP_ArrowDown, 51},
        {(unsigned int)QStyle::SP_ArrowLeft, 52},
        {(unsigned int)QStyle::SP_ArrowRight, 53},
        {(unsigned int)QStyle::SP_ArrowBack, 54},
        {(unsigned int)QStyle::SP_ArrowForward, 55},
        {(unsigned int)QStyle::SP_DirHomeIcon, 56},
        {(unsigned int)QStyle::SP_CommandLink, 57},
        {(unsigned int)QStyle::SP_VistaShield, 58},
        {(unsigned int)QStyle::SP_BrowserReload, 59},
        {(unsigned int)QStyle::SP_BrowserStop, 60},
        {(unsigned int)QStyle::SP_MediaPlay, 61},
        {(unsigned int)QStyle::SP_MediaStop, 62},
        {(unsigned int)QStyle::SP_MediaPause, 63},
        {(unsigned int)QStyle::SP_MediaSkipForward, 64},
        {(unsigned int)QStyle::SP_MediaSkipBackward, 65},
        {(unsigned int)QStyle::SP_MediaSeekForward, 66},
        {(unsigned int)QStyle::SP_MediaSeekBackward, 67},
        {(unsigned int)QStyle::SP_MediaVolume, 68},
        {(unsigned int)QStyle::SP_MediaVolumeMuted, 69},
        {(unsigned int)QStyle::SP_LineEditClearButton, 70},
        {(unsigned int)QStyle::SP_CustomBase, 4026531840},
    };

    for (int i = 0; i < 72; ++i) {
        s_standard_pixmap_lookup[standard_pixmap_vals[i].key] =
            (unsigned int)standard_pixmap_vals[i].val;
    }

    static KeyVal state_vals[] = {
        {(unsigned int)QIcon::On, 0},
        {(unsigned int)QIcon::Off, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_state_lookup[state_vals[i].key] = (unsigned int)state_vals[i].val;
    }

    static KeyVal state_flag_vals[] = {
        {(unsigned int)QStyle::State_None, 0},
        {(unsigned int)QStyle::State_Enabled, 1},
        {(unsigned int)QStyle::State_Raised, 2},
        {(unsigned int)QStyle::State_Sunken, 4},
        {(unsigned int)QStyle::State_Off, 8},
        {(unsigned int)QStyle::State_NoChange, 16},
        {(unsigned int)QStyle::State_On, 32},
        {(unsigned int)QStyle::State_DownArrow, 64},
        {(unsigned int)QStyle::State_Horizontal, 128},
        {(unsigned int)QStyle::State_HasFocus, 256},
        {(unsigned int)QStyle::State_Top, 512},
        {(unsigned int)QStyle::State_Bottom, 1024},
        {(unsigned int)QStyle::State_FocusAtBorder, 2048},
        {(unsigned int)QStyle::State_AutoRaise, 4096},
        {(unsigned int)QStyle::State_MouseOver, 8192},
        {(unsigned int)QStyle::State_UpArrow, 16384},
        {(unsigned int)QStyle::State_Selected, 32768},
        {(unsigned int)QStyle::State_Active, 65536},
        {(unsigned int)QStyle::State_Window, 131072},
        {(unsigned int)QStyle::State_Open, 262144},
        {(unsigned int)QStyle::State_Children, 524288},
        {(unsigned int)QStyle::State_Item, 1048576},
        {(unsigned int)QStyle::State_Sibling, 2097152},
        {(unsigned int)QStyle::State_Editing, 4194304},
        {(unsigned int)QStyle::State_KeyboardFocusChange, 8388608},
        {(unsigned int)QStyle::State_ReadOnly, 33554432},
        {(unsigned int)QStyle::State_Small, 67108864},
        {(unsigned int)QStyle::State_Mini, 134217728},
    };

    for (int i = 0; i < 28; ++i) {
        s_state_flag_lookup[state_flag_vals[i].key] =
            (unsigned int)state_flag_vals[i].val;
    }

    static KeyVal stretch_vals[] = {
        {(unsigned int)QFont::AnyStretch, 0},
        {(unsigned int)QFont::UltraCondensed, 50},
        {(unsigned int)QFont::ExtraCondensed, 62},
        {(unsigned int)QFont::Condensed, 75},
        {(unsigned int)QFont::SemiCondensed, 87},
        {(unsigned int)QFont::Unstretched, 100},
        {(unsigned int)QFont::SemiExpanded, 112},
        {(unsigned int)QFont::Expanded, 125},
        {(unsigned int)QFont::ExtraExpanded, 150},
        {(unsigned int)QFont::UltraExpanded, 200},
    };

    for (int i = 0; i < 10; ++i) {
        s_stretch_lookup[stretch_vals[i].key] =
            (unsigned int)stretch_vals[i].val;
    }

    static KeyVal style_vals[] = {
        {(unsigned int)QFont::StyleNormal, 0},
        {(unsigned int)QFont::StyleItalic, 1},
        {(unsigned int)QFont::StyleOblique, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_style_lookup[style_vals[i].key] = (unsigned int)style_vals[i].val;
    }

    static KeyVal style_hint_vals[] = {
        {(unsigned int)QStyle::SH_EtchDisabledText, 0},
        {(unsigned int)QStyle::SH_DitherDisabledText, 1},
        {(unsigned int)QStyle::SH_ScrollBar_MiddleClickAbsolutePosition, 2},
        {(unsigned int)QStyle::SH_ScrollBar_ScrollWhenPointerLeavesControl, 3},
        {(unsigned int)QStyle::SH_TabBar_SelectMouseType, 4},
        {(unsigned int)QStyle::SH_TabBar_Alignment, 5},
        {(unsigned int)QStyle::SH_Header_ArrowAlignment, 6},
        {(unsigned int)QStyle::SH_Slider_SnapToValue, 7},
        {(unsigned int)QStyle::SH_Slider_SloppyKeyEvents, 8},
        {(unsigned int)QStyle::SH_ProgressDialog_CenterCancelButton, 9},
        {(unsigned int)QStyle::SH_ProgressDialog_TextLabelAlignment, 10},
        {(unsigned int)QStyle::SH_PrintDialog_RightAlignButtons, 11},
        {(unsigned int)QStyle::SH_MainWindow_SpaceBelowMenuBar, 12},
        {(unsigned int)QStyle::SH_FontDialog_SelectAssociatedText, 13},
        {(unsigned int)QStyle::SH_Menu_AllowActiveAndDisabled, 14},
        {(unsigned int)QStyle::SH_Menu_SpaceActivatesItem, 15},
        {(unsigned int)QStyle::SH_Menu_SubMenuPopupDelay, 16},
        {(unsigned int)QStyle::SH_ScrollView_FrameOnlyAroundContents, 17},
        {(unsigned int)QStyle::SH_MenuBar_AltKeyNavigation, 18},
        {(unsigned int)QStyle::SH_ComboBox_ListMouseTracking, 19},
        {(unsigned int)QStyle::SH_Menu_MouseTracking, 20},
        {(unsigned int)QStyle::SH_MenuBar_MouseTracking, 21},
        {(unsigned int)QStyle::SH_ItemView_ChangeHighlightOnFocus, 22},
        {(unsigned int)QStyle::SH_Widget_ShareActivation, 23},
        {(unsigned int)QStyle::SH_Workspace_FillSpaceOnMaximize, 24},
        {(unsigned int)QStyle::SH_ComboBox_Popup, 25},
        {(unsigned int)QStyle::SH_TitleBar_NoBorder, 26},
        {(unsigned int)QStyle::SH_Slider_StopMouseOverSlider, 27},
        {(unsigned int)QStyle::SH_ScrollBar_StopMouseOverSlider, 27},
        {(unsigned int)QStyle::SH_BlinkCursorWhenTextSelected, 28},
        {(unsigned int)QStyle::SH_RichText_FullWidthSelection, 29},
        {(unsigned int)QStyle::SH_Menu_Scrollable, 30},
        {(unsigned int)QStyle::SH_GroupBox_TextLabelVerticalAlignment, 31},
        {(unsigned int)QStyle::SH_GroupBox_TextLabelColor, 32},
        {(unsigned int)QStyle::SH_Menu_SloppySubMenus, 33},
        {(unsigned int)QStyle::SH_Table_GridLineColor, 34},
        {(unsigned int)QStyle::SH_LineEdit_PasswordCharacter, 35},
        {(unsigned int)QStyle::SH_DialogButtons_DefaultButton, 36},
        {(unsigned int)QStyle::SH_ToolBox_SelectedPageTitleBold, 37},
        {(unsigned int)QStyle::SH_TabBar_PreferNoArrows, 38},
        {(unsigned int)QStyle::SH_ScrollBar_LeftClickAbsolutePosition, 39},
        {(unsigned int)QStyle::SH_ListViewExpand_SelectMouseType, 40},
        {(unsigned int)QStyle::SH_UnderlineShortcut, 41},
        {(unsigned int)QStyle::SH_SpinBox_AnimateButton, 42},
        {(unsigned int)QStyle::SH_SpinBox_KeyPressAutoRepeatRate, 43},
        {(unsigned int)QStyle::SH_SpinBox_ClickAutoRepeatRate, 44},
        {(unsigned int)QStyle::SH_Menu_FillScreenWithScroll, 45},
        {(unsigned int)QStyle::SH_ToolTipLabel_Opacity, 46},
        {(unsigned int)QStyle::SH_DrawMenuBarSeparator, 47},
        {(unsigned int)QStyle::SH_TitleBar_ModifyNotification, 48},
        {(unsigned int)QStyle::SH_Button_FocusPolicy, 49},
        {(unsigned int)QStyle::SH_MessageBox_UseBorderForButtonSpacing, 50},
        {(unsigned int)QStyle::SH_TitleBar_AutoRaise, 51},
        {(unsigned int)QStyle::SH_ToolButton_PopupDelay, 52},
        {(unsigned int)QStyle::SH_FocusFrame_Mask, 53},
        {(unsigned int)QStyle::SH_RubberBand_Mask, 54},
        {(unsigned int)QStyle::SH_WindowFrame_Mask, 55},
        {(unsigned int)QStyle::SH_SpinControls_DisableOnBounds, 56},
        {(unsigned int)QStyle::SH_Dial_BackgroundRole, 57},
        {(unsigned int)QStyle::SH_ComboBox_LayoutDirection, 58},
        {(unsigned int)QStyle::SH_ItemView_EllipsisLocation, 59},
        {(unsigned int)QStyle::SH_ItemView_ShowDecorationSelected, 60},
        {(unsigned int)QStyle::SH_ItemView_ActivateItemOnSingleClick, 61},
        {(unsigned int)QStyle::SH_ScrollBar_ContextMenu, 62},
        {(unsigned int)QStyle::SH_ScrollBar_RollBetweenButtons, 63},
        {(unsigned int)QStyle::SH_Slider_AbsoluteSetButtons, 64},
        {(unsigned int)QStyle::SH_Slider_PageSetButtons, 65},
        {(unsigned int)QStyle::SH_Menu_KeyboardSearch, 66},
        {(unsigned int)QStyle::SH_TabBar_ElideMode, 67},
        {(unsigned int)QStyle::SH_DialogButtonLayout, 68},
        {(unsigned int)QStyle::SH_ComboBox_PopupFrameStyle, 69},
        {(unsigned int)QStyle::SH_MessageBox_TextInteractionFlags, 70},
        {(unsigned int)QStyle::SH_DialogButtonBox_ButtonsHaveIcons, 71},
        {(unsigned int)QStyle::SH_SpellCheckUnderlineStyle, 72},
        {(unsigned int)QStyle::SH_MessageBox_CenterButtons, 73},
        {(unsigned int)QStyle::SH_Menu_SelectionWrap, 74},
        {(unsigned int)QStyle::SH_ItemView_MovementWithoutUpdatingSelection,
         75},
        {(unsigned int)QStyle::SH_ToolTip_Mask, 76},
        {(unsigned int)QStyle::SH_FocusFrame_AboveWidget, 77},
        {(unsigned int)QStyle::SH_TextControl_FocusIndicatorTextCharFormat, 78},
        {(unsigned int)QStyle::SH_WizardStyle, 79},
        {(unsigned int)QStyle::SH_ItemView_ArrowKeysNavigateIntoChildren, 80},
        {(unsigned int)QStyle::SH_Menu_Mask, 81},
        {(unsigned int)QStyle::SH_Menu_FlashTriggeredItem, 82},
        {(unsigned int)QStyle::SH_Menu_FadeOutOnHide, 83},
        {(unsigned int)QStyle::SH_SpinBox_ClickAutoRepeatThreshold, 84},
        {(unsigned int)
             QStyle::SH_ItemView_PaintAlternatingRowColorsForEmptyArea,
         85},
        {(unsigned int)QStyle::SH_FormLayoutWrapPolicy, 86},
        {(unsigned int)QStyle::SH_TabWidget_DefaultTabPosition, 87},
        {(unsigned int)QStyle::SH_ToolBar_Movable, 88},
        {(unsigned int)QStyle::SH_FormLayoutFieldGrowthPolicy, 89},
        {(unsigned int)QStyle::SH_FormLayoutFormAlignment, 90},
        {(unsigned int)QStyle::SH_FormLayoutLabelAlignment, 91},
        {(unsigned int)QStyle::SH_ItemView_DrawDelegateFrame, 92},
        {(unsigned int)QStyle::SH_TabBar_CloseButtonPosition, 93},
        {(unsigned int)QStyle::SH_DockWidget_ButtonsHaveFrame, 94},
        {(unsigned int)QStyle::SH_ToolButtonStyle, 95},
        {(unsigned int)QStyle::SH_RequestSoftwareInputPanel, 96},
        {(unsigned int)QStyle::SH_ScrollBar_Transient, 97},
        {(unsigned int)QStyle::SH_Menu_SupportsSections, 98},
        {(unsigned int)QStyle::SH_ToolTip_WakeUpDelay, 99},
        {(unsigned int)QStyle::SH_ToolTip_FallAsleepDelay, 100},
        {(unsigned int)QStyle::SH_Widget_Animate, 101},
        {(unsigned int)QStyle::SH_Splitter_OpaqueResize, 102},
        {(unsigned int)QStyle::SH_ComboBox_UseNativePopup, 103},
        {(unsigned int)QStyle::SH_LineEdit_PasswordMaskDelay, 104},
        {(unsigned int)QStyle::SH_TabBar_ChangeCurrentDelay, 105},
        {(unsigned int)QStyle::SH_Menu_SubMenuUniDirection, 106},
        {(unsigned int)QStyle::SH_Menu_SubMenuUniDirectionFailCount, 107},
        {(unsigned int)QStyle::SH_Menu_SubMenuSloppySelectOtherActions, 108},
        {(unsigned int)QStyle::SH_Menu_SubMenuSloppyCloseTimeout, 109},
        {(unsigned int)QStyle::SH_Menu_SubMenuResetWhenReenteringParent, 110},
        {(unsigned int)QStyle::SH_Menu_SubMenuDontStartSloppyOnLeave, 111},
        {(unsigned int)QStyle::SH_ItemView_ScrollMode, 112},
        {(unsigned int)QStyle::SH_TitleBar_ShowToolTipsOnButtons, 113},
        {(unsigned int)QStyle::SH_Widget_Animation_Duration, 114},
        {(unsigned int)QStyle::SH_ComboBox_AllowWheelScrolling, 115},
        {(unsigned int)QStyle::SH_SpinBox_ButtonsInsideFrame, 116},
        {(unsigned int)QStyle::SH_CustomBase, 4026531840},
    };

    for (int i = 0; i < 119; ++i) {
        s_style_hint_lookup[style_hint_vals[i].key] =
            (unsigned int)style_hint_vals[i].val;
    }

    static KeyVal style_strategy_vals[] = {
        {(unsigned int)QFont::PreferDefault, 1},
        {(unsigned int)QFont::PreferBitmap, 2},
        {(unsigned int)QFont::PreferDevice, 4},
        {(unsigned int)QFont::PreferOutline, 8},
        {(unsigned int)QFont::ForceOutline, 16},
        {(unsigned int)QFont::PreferMatch, 32},
        {(unsigned int)QFont::PreferQuality, 64},
        {(unsigned int)QFont::PreferAntialias, 128},
        {(unsigned int)QFont::NoAntialias, 256},
        {(unsigned int)QFont::OpenGLCompatible, 512},
        {(unsigned int)QFont::ForceIntegerMetrics, 1024},
        {(unsigned int)QFont::NoSubpixelAntialias, 2048},
        {(unsigned int)QFont::PreferNoShaping, 4096},
        {(unsigned int)QFont::NoFontMerging, 32768},
    };

    for (int i = 0; i < 14; ++i) {
        s_style_strategy_lookup[style_strategy_vals[i].key] =
            (unsigned int)style_strategy_vals[i].val;
    }

    static KeyVal sub_control_vals[] = {
        {(unsigned int)QStyle::SC_None, 0},
        {(unsigned int)QStyle::SC_ScrollBarAddLine, 1},
        {(unsigned int)QStyle::SC_ScrollBarSubLine, 2},
        {(unsigned int)QStyle::SC_ScrollBarAddPage, 4},
        {(unsigned int)QStyle::SC_ScrollBarSubPage, 8},
        {(unsigned int)QStyle::SC_ScrollBarFirst, 16},
        {(unsigned int)QStyle::SC_ScrollBarLast, 32},
        {(unsigned int)QStyle::SC_ScrollBarSlider, 64},
        {(unsigned int)QStyle::SC_ScrollBarGroove, 128},
        {(unsigned int)QStyle::SC_SpinBoxUp, 1},
        {(unsigned int)QStyle::SC_SpinBoxDown, 2},
        {(unsigned int)QStyle::SC_SpinBoxFrame, 4},
        {(unsigned int)QStyle::SC_SpinBoxEditField, 8},
        {(unsigned int)QStyle::SC_ComboBoxFrame, 1},
        {(unsigned int)QStyle::SC_ComboBoxEditField, 2},
        {(unsigned int)QStyle::SC_ComboBoxArrow, 4},
        {(unsigned int)QStyle::SC_ComboBoxListBoxPopup, 8},
        {(unsigned int)QStyle::SC_SliderGroove, 1},
        {(unsigned int)QStyle::SC_SliderHandle, 2},
        {(unsigned int)QStyle::SC_SliderTickmarks, 4},
        {(unsigned int)QStyle::SC_ToolButton, 1},
        {(unsigned int)QStyle::SC_ToolButtonMenu, 2},
        {(unsigned int)QStyle::SC_TitleBarSysMenu, 1},
        {(unsigned int)QStyle::SC_TitleBarMinButton, 2},
        {(unsigned int)QStyle::SC_TitleBarMaxButton, 4},
        {(unsigned int)QStyle::SC_TitleBarCloseButton, 8},
        {(unsigned int)QStyle::SC_TitleBarNormalButton, 16},
        {(unsigned int)QStyle::SC_TitleBarShadeButton, 32},
        {(unsigned int)QStyle::SC_TitleBarUnshadeButton, 64},
        {(unsigned int)QStyle::SC_TitleBarContextHelpButton, 128},
        {(unsigned int)QStyle::SC_TitleBarLabel, 256},
        {(unsigned int)QStyle::SC_DialGroove, 1},
        {(unsigned int)QStyle::SC_DialHandle, 2},
        {(unsigned int)QStyle::SC_DialTickmarks, 4},
        {(unsigned int)QStyle::SC_GroupBoxCheckBox, 1},
        {(unsigned int)QStyle::SC_GroupBoxLabel, 2},
        {(unsigned int)QStyle::SC_GroupBoxContents, 4},
        {(unsigned int)QStyle::SC_GroupBoxFrame, 8},
        {(unsigned int)QStyle::SC_MdiMinButton, 1},
        {(unsigned int)QStyle::SC_MdiNormalButton, 2},
        {(unsigned int)QStyle::SC_MdiCloseButton, 4},
        {(unsigned int)QStyle::SC_CustomBase, 4026531840},
        {(unsigned int)QStyle::SC_All, 4294967295},
    };

    for (int i = 0; i < 43; ++i) {
        s_sub_control_lookup[sub_control_vals[i].key] =
            (unsigned int)sub_control_vals[i].val;
    }

    static KeyVal sub_element_vals[] = {
        {(unsigned int)QStyle::SE_PushButtonContents, 0},
        {(unsigned int)QStyle::SE_PushButtonFocusRect, 1},
        {(unsigned int)QStyle::SE_CheckBoxIndicator, 2},
        {(unsigned int)QStyle::SE_CheckBoxContents, 3},
        {(unsigned int)QStyle::SE_CheckBoxFocusRect, 4},
        {(unsigned int)QStyle::SE_CheckBoxClickRect, 5},
        {(unsigned int)QStyle::SE_RadioButtonIndicator, 6},
        {(unsigned int)QStyle::SE_RadioButtonContents, 7},
        {(unsigned int)QStyle::SE_RadioButtonFocusRect, 8},
        {(unsigned int)QStyle::SE_RadioButtonClickRect, 9},
        {(unsigned int)QStyle::SE_ComboBoxFocusRect, 10},
        {(unsigned int)QStyle::SE_SliderFocusRect, 11},
        {(unsigned int)QStyle::SE_ProgressBarGroove, 12},
        {(unsigned int)QStyle::SE_ProgressBarContents, 13},
        {(unsigned int)QStyle::SE_ProgressBarLabel, 14},
        {(unsigned int)QStyle::SE_ToolBoxTabContents, 15},
        {(unsigned int)QStyle::SE_HeaderLabel, 16},
        {(unsigned int)QStyle::SE_HeaderArrow, 17},
        {(unsigned int)QStyle::SE_TabWidgetTabBar, 18},
        {(unsigned int)QStyle::SE_TabWidgetTabPane, 19},
        {(unsigned int)QStyle::SE_TabWidgetTabContents, 20},
        {(unsigned int)QStyle::SE_TabWidgetLeftCorner, 21},
        {(unsigned int)QStyle::SE_TabWidgetRightCorner, 22},
        {(unsigned int)QStyle::SE_ViewItemCheckIndicator, 23},
        {(unsigned int)QStyle::SE_ItemViewItemCheckIndicator, 23},
        {(unsigned int)QStyle::SE_TabBarTearIndicator, 24},
        {(unsigned int)QStyle::SE_TabBarTearIndicatorLeft, 24},
        {(unsigned int)QStyle::SE_TreeViewDisclosureItem, 25},
        {(unsigned int)QStyle::SE_LineEditContents, 26},
        {(unsigned int)QStyle::SE_FrameContents, 27},
        {(unsigned int)QStyle::SE_DockWidgetCloseButton, 28},
        {(unsigned int)QStyle::SE_DockWidgetFloatButton, 29},
        {(unsigned int)QStyle::SE_DockWidgetTitleBarText, 30},
        {(unsigned int)QStyle::SE_DockWidgetIcon, 31},
        {(unsigned int)QStyle::SE_CheckBoxLayoutItem, 32},
        {(unsigned int)QStyle::SE_ComboBoxLayoutItem, 33},
        {(unsigned int)QStyle::SE_DateTimeEditLayoutItem, 34},
        {(unsigned int)QStyle::SE_DialogButtonBoxLayoutItem, 35},
        {(unsigned int)QStyle::SE_LabelLayoutItem, 36},
        {(unsigned int)QStyle::SE_ProgressBarLayoutItem, 37},
        {(unsigned int)QStyle::SE_PushButtonLayoutItem, 38},
        {(unsigned int)QStyle::SE_RadioButtonLayoutItem, 39},
        {(unsigned int)QStyle::SE_SliderLayoutItem, 40},
        {(unsigned int)QStyle::SE_SpinBoxLayoutItem, 41},
        {(unsigned int)QStyle::SE_ToolButtonLayoutItem, 42},
        {(unsigned int)QStyle::SE_FrameLayoutItem, 43},
        {(unsigned int)QStyle::SE_GroupBoxLayoutItem, 44},
        {(unsigned int)QStyle::SE_TabWidgetLayoutItem, 45},
        {(unsigned int)QStyle::SE_ItemViewItemDecoration, 46},
        {(unsigned int)QStyle::SE_ItemViewItemText, 47},
        {(unsigned int)QStyle::SE_ItemViewItemFocusRect, 48},
        {(unsigned int)QStyle::SE_TabBarTabLeftButton, 49},
        {(unsigned int)QStyle::SE_TabBarTabRightButton, 50},
        {(unsigned int)QStyle::SE_TabBarTabText, 51},
        {(unsigned int)QStyle::SE_ShapedFrameContents, 52},
        {(unsigned int)QStyle::SE_ToolBarHandle, 53},
        {(unsigned int)QStyle::SE_TabBarScrollLeftButton, 54},
        {(unsigned int)QStyle::SE_TabBarScrollRightButton, 55},
        {(unsigned int)QStyle::SE_TabBarTearIndicatorRight, 56},
        {(unsigned int)QStyle::SE_CustomBase, 4026531840},
    };

    for (int i = 0; i < 60; ++i) {
        s_sub_element_lookup[sub_element_vals[i].key] =
            (unsigned int)sub_element_vals[i].val;
    }

    static KeyVal surface_class_vals[] = {
        {(unsigned int)QSurface::Window, 0},
        {(unsigned int)QSurface::Offscreen, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_surface_class_lookup[surface_class_vals[i].key] =
            (unsigned int)surface_class_vals[i].val;
    }

    static KeyVal surface_type_vals[] = {
        {(unsigned int)QSurface::RasterSurface, 0},
        {(unsigned int)QSurface::OpenGLSurface, 1},
        {(unsigned int)QSurface::RasterGLSurface, 2},
        {(unsigned int)QSurface::OpenVGSurface, 3},
        {(unsigned int)QSurface::VulkanSurface, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_surface_type_lookup[surface_type_vals[i].key] =
            (unsigned int)surface_type_vals[i].val;
    }

    static KeyVal swap_behavior_vals[] = {
        {(unsigned int)QSurfaceFormat::DefaultSwapBehavior, 0},
        {(unsigned int)QSurfaceFormat::SingleBuffer, 1},
        {(unsigned int)QSurfaceFormat::DoubleBuffer, 2},
        {(unsigned int)QSurfaceFormat::TripleBuffer, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_swap_behavior_lookup[swap_behavior_vals[i].key] =
            (unsigned int)swap_behavior_vals[i].val;
    }

    static KeyVal tab_focus_behavior_vals[] = {
        {(unsigned int)Qt::NoTabFocus, 0},
        {(unsigned int)Qt::TabFocusTextControls, 1},
        {(unsigned int)Qt::TabFocusListControls, 2},
        {(unsigned int)Qt::TabFocusAllControls, 255},
    };

    for (int i = 0; i < 4; ++i) {
        s_tab_focus_behavior_lookup[tab_focus_behavior_vals[i].key] =
            (unsigned int)tab_focus_behavior_vals[i].val;
    }

    static KeyVal tablet_device_vals[] = {
        {(unsigned int)QTabletEvent::NoDevice, 0},
        {(unsigned int)QTabletEvent::Puck, 1},
        {(unsigned int)QTabletEvent::Stylus, 2},
        {(unsigned int)QTabletEvent::Airbrush, 3},
        {(unsigned int)QTabletEvent::FourDMouse, 4},
        {(unsigned int)QTabletEvent::XFreeEraser, 5},
        {(unsigned int)QTabletEvent::RotationStylus, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_tablet_device_lookup[tablet_device_vals[i].key] =
            (unsigned int)tablet_device_vals[i].val;
    }

    static KeyVal text_elide_mode_vals[] = {
        {(unsigned int)Qt::ElideLeft, 0},
        {(unsigned int)Qt::ElideRight, 1},
        {(unsigned int)Qt::ElideMiddle, 2},
        {(unsigned int)Qt::ElideNone, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_text_elide_mode_lookup[text_elide_mode_vals[i].key] =
            (unsigned int)text_elide_mode_vals[i].val;
    }

    static KeyVal text_flag_vals[] = {
        {(unsigned int)Qt::TextSingleLine, 256},
        {(unsigned int)Qt::TextDontClip, 512},
        {(unsigned int)Qt::TextExpandTabs, 1024},
        {(unsigned int)Qt::TextShowMnemonic, 2048},
        {(unsigned int)Qt::TextWordWrap, 4096},
        {(unsigned int)Qt::TextWrapAnywhere, 8192},
        {(unsigned int)Qt::TextDontPrint, 16384},
        {(unsigned int)Qt::TextIncludeTrailingSpaces, 134217728},
        {(unsigned int)Qt::TextHideMnemonic, 32768},
        {(unsigned int)Qt::TextJustificationForced, 65536},
        {(unsigned int)Qt::TextForceLeftToRight, 131072},
        {(unsigned int)Qt::TextForceRightToLeft, 262144},
        {(unsigned int)Qt::TextLongestVariant, 524288},
        {(unsigned int)Qt::TextBypassShaping, 1048576},
    };

    for (int i = 0; i < 14; ++i) {
        s_text_flag_lookup[text_flag_vals[i].key] =
            (unsigned int)text_flag_vals[i].val;
    }

    static KeyVal text_format_vals[] = {
        {(unsigned int)Qt::PlainText, 0},
        {(unsigned int)Qt::RichText, 1},
        {(unsigned int)Qt::AutoText, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_text_format_lookup[text_format_vals[i].key] =
            (unsigned int)text_format_vals[i].val;
    }

    static KeyVal text_interaction_flag_vals[] = {
        {(unsigned int)Qt::NoTextInteraction, 0},
        {(unsigned int)Qt::TextSelectableByMouse, 1},
        {(unsigned int)Qt::TextSelectableByKeyboard, 2},
        {(unsigned int)Qt::LinksAccessibleByMouse, 4},
        {(unsigned int)Qt::LinksAccessibleByKeyboard, 8},
        {(unsigned int)Qt::TextEditable, 16},
        {(unsigned int)Qt::TextEditorInteraction, 19},
        {(unsigned int)Qt::TextBrowserInteraction, 13},
    };

    for (int i = 0; i < 8; ++i) {
        s_text_interaction_flag_lookup[text_interaction_flag_vals[i].key] =
            (unsigned int)text_interaction_flag_vals[i].val;
    }

    static KeyVal tile_rule_vals[] = {
        {(unsigned int)Qt::StretchTile, 0},
        {(unsigned int)Qt::RepeatTile, 1},
        {(unsigned int)Qt::RoundTile, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_tile_rule_lookup[tile_rule_vals[i].key] =
            (unsigned int)tile_rule_vals[i].val;
    }

    static KeyVal time_spec_vals[] = {
        {(unsigned int)Qt::LocalTime, 0},
        {(unsigned int)Qt::UTC, 1},
        {(unsigned int)Qt::OffsetFromUTC, 2},
        {(unsigned int)Qt::TimeZone, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_time_spec_lookup[time_spec_vals[i].key] =
            (unsigned int)time_spec_vals[i].val;
    }

    static KeyVal timer_type_vals[] = {
        {(unsigned int)Qt::PreciseTimer, 0},
        {(unsigned int)Qt::CoarseTimer, 1},
        {(unsigned int)Qt::VeryCoarseTimer, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_timer_type_lookup[timer_type_vals[i].key] =
            (unsigned int)timer_type_vals[i].val;
    }

    static KeyVal tool_bar_area_vals[] = {
        {(unsigned int)Qt::LeftToolBarArea, 1},
        {(unsigned int)Qt::RightToolBarArea, 2},
        {(unsigned int)Qt::TopToolBarArea, 4},
        {(unsigned int)Qt::BottomToolBarArea, 8},
        {(unsigned int)Qt::ToolBarArea_Mask, 15},
        {(unsigned int)Qt::AllToolBarAreas, 15},
        {(unsigned int)Qt::NoToolBarArea, 0},
    };

    for (int i = 0; i < 7; ++i) {
        s_tool_bar_area_lookup[tool_bar_area_vals[i].key] =
            (unsigned int)tool_bar_area_vals[i].val;
    }

    static KeyVal tool_bar_area_sizes_vals[] = {
        {(unsigned int)Qt::NToolBarAreas, 4},
    };

    for (int i = 0; i < 1; ++i) {
        s_tool_bar_area_sizes_lookup[tool_bar_area_sizes_vals[i].key] =
            (unsigned int)tool_bar_area_sizes_vals[i].val;
    }

    static KeyVal tool_button_popup_mode_vals[] = {
        {(unsigned int)QToolButton::DelayedPopup, 0},
        {(unsigned int)QToolButton::MenuButtonPopup, 1},
        {(unsigned int)QToolButton::InstantPopup, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_tool_button_popup_mode_lookup[tool_button_popup_mode_vals[i].key] =
            (unsigned int)tool_button_popup_mode_vals[i].val;
    }

    static KeyVal tool_button_style_vals[] = {
        {(unsigned int)Qt::ToolButtonIconOnly, 0},
        {(unsigned int)Qt::ToolButtonTextOnly, 1},
        {(unsigned int)Qt::ToolButtonTextBesideIcon, 2},
        {(unsigned int)Qt::ToolButtonTextUnderIcon, 3},
        {(unsigned int)Qt::ToolButtonFollowStyle, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_tool_button_style_lookup[tool_button_style_vals[i].key] =
            (unsigned int)tool_button_style_vals[i].val;
    }

    static KeyVal touch_point_state_vals[] = {
        {(unsigned int)Qt::TouchPointPressed, 1},
        {(unsigned int)Qt::TouchPointMoved, 2},
        {(unsigned int)Qt::TouchPointStationary, 4},
        {(unsigned int)Qt::TouchPointReleased, 8},
    };

    for (int i = 0; i < 4; ++i) {
        s_touch_point_state_lookup[touch_point_state_vals[i].key] =
            (unsigned int)touch_point_state_vals[i].val;
    }

    static KeyVal transformation_mode_vals[] = {
        {(unsigned int)Qt::FastTransformation, 0},
        {(unsigned int)Qt::SmoothTransformation, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_transformation_mode_lookup[transformation_mode_vals[i].key] =
            (unsigned int)transformation_mode_vals[i].val;
    }

    static KeyVal transformation_type_vals[] = {
        {(unsigned int)QTransform::TxNone, 0},
        {(unsigned int)QTransform::TxTranslate, 1},
        {(unsigned int)QTransform::TxScale, 2},
        {(unsigned int)QTransform::TxRotate, 4},
        {(unsigned int)QTransform::TxShear, 8},
        {(unsigned int)QTransform::TxProject, 16},
    };

    for (int i = 0; i < 6; ++i) {
        s_transformation_type_lookup[transformation_type_vals[i].key] =
            (unsigned int)transformation_type_vals[i].val;
    }

    static KeyVal type_vals[] = {
        {(unsigned int)QPaintEngine::X11, 0},
        {(unsigned int)QPaintEngine::Windows, 1},
        {(unsigned int)QPaintEngine::QuickDraw, 2},
        {(unsigned int)QPaintEngine::CoreGraphics, 3},
        {(unsigned int)QPaintEngine::MacPrinter, 4},
        {(unsigned int)QPaintEngine::QWindowSystem, 5},
        {(unsigned int)QPaintEngine::PostScript, 6},
        {(unsigned int)QPaintEngine::OpenGL, 7},
        {(unsigned int)QPaintEngine::Picture, 8},
        {(unsigned int)QPaintEngine::SVG, 9},
        {(unsigned int)QPaintEngine::Raster, 10},
        {(unsigned int)QPaintEngine::Direct3D, 11},
        {(unsigned int)QPaintEngine::Pdf, 12},
        {(unsigned int)QPaintEngine::OpenVG, 13},
        {(unsigned int)QPaintEngine::OpenGL2, 14},
        {(unsigned int)QPaintEngine::PaintBuffer, 15},
        {(unsigned int)QPaintEngine::Blitter, 16},
        {(unsigned int)QPaintEngine::Direct2D, 17},
        {(unsigned int)QPaintEngine::User, 50},
        {(unsigned int)QPaintEngine::MaxUser, 100},
    };

    for (int i = 0; i < 20; ++i) {
        s_type_lookup[type_vals[i].key] = (unsigned int)type_vals[i].val;
    }

    static KeyVal type_interpretation_vals[] = {
        {(unsigned int)QPixelFormat::UnsignedInteger, 0},
        {(unsigned int)QPixelFormat::UnsignedShort, 1},
        {(unsigned int)QPixelFormat::UnsignedByte, 2},
        {(unsigned int)QPixelFormat::FloatingPoint, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_type_interpretation_lookup[type_interpretation_vals[i].key] =
            (unsigned int)type_interpretation_vals[i].val;
    }

    static KeyVal ui_effect_vals[] = {
        {(unsigned int)Qt::UI_General, 0},
        {(unsigned int)Qt::UI_AnimateMenu, 1},
        {(unsigned int)Qt::UI_FadeMenu, 2},
        {(unsigned int)Qt::UI_AnimateCombo, 3},
        {(unsigned int)Qt::UI_AnimateTooltip, 4},
        {(unsigned int)Qt::UI_FadeTooltip, 5},
        {(unsigned int)Qt::UI_AnimateToolBox, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_ui_effect_lookup[ui_effect_vals[i].key] =
            (unsigned int)ui_effect_vals[i].val;
    }

    static KeyVal visibility_vals[] = {
        {(unsigned int)QWindow::Hidden, 0},
        {(unsigned int)QWindow::AutomaticVisibility, 1},
        {(unsigned int)QWindow::Windowed, 2},
        {(unsigned int)QWindow::Minimized, 3},
        {(unsigned int)QWindow::Maximized, 4},
        {(unsigned int)QWindow::FullScreen, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_visibility_lookup[visibility_vals[i].key] =
            (unsigned int)visibility_vals[i].val;
    }

    static KeyVal weight_vals[] = {
        {(unsigned int)QFont::Thin, 0},
        {(unsigned int)QFont::ExtraLight, 12},
        {(unsigned int)QFont::Light, 25},
        {(unsigned int)QFont::Normal, 50},
        {(unsigned int)QFont::Medium, 57},
        {(unsigned int)QFont::DemiBold, 63},
        {(unsigned int)QFont::Bold, 75},
        {(unsigned int)QFont::ExtraBold, 81},
        {(unsigned int)QFont::Black, 87},
    };

    for (int i = 0; i < 9; ++i) {
        s_weight_lookup[weight_vals[i].key] = (unsigned int)weight_vals[i].val;
    }

    static KeyVal wheel_event_fix_me_enums_vals[] = {
        {(unsigned int)QWheelEvent::DefaultDeltasPerStep, 120},
    };

    for (int i = 0; i < 1; ++i) {
        s_wheel_event_fix_me_enums_lookup[wheel_event_fix_me_enums_vals[i]
                                              .key] =
            (unsigned int)wheel_event_fix_me_enums_vals[i].val;
    }

    static KeyVal white_space_mode_vals[] = {
        {(unsigned int)Qt::WhiteSpaceNormal, 0},
        {(unsigned int)Qt::WhiteSpacePre, 1},
        {(unsigned int)Qt::WhiteSpaceNoWrap, 2},
        {(unsigned int)Qt::WhiteSpaceModeUndefined, 4294967295},
    };

    for (int i = 0; i < 4; ++i) {
        s_white_space_mode_lookup[white_space_mode_vals[i].key] =
            (unsigned int)white_space_mode_vals[i].val;
    }

    static KeyVal widget_attribute_vals[] = {
        {(unsigned int)Qt::WA_Disabled, 0},
        {(unsigned int)Qt::WA_UnderMouse, 1},
        {(unsigned int)Qt::WA_MouseTracking, 2},
        {(unsigned int)Qt::WA_ContentsPropagated, 3},
        {(unsigned int)Qt::WA_OpaquePaintEvent, 4},
        {(unsigned int)Qt::WA_NoBackground, 4},
        {(unsigned int)Qt::WA_StaticContents, 5},
        {(unsigned int)Qt::WA_LaidOut, 7},
        {(unsigned int)Qt::WA_PaintOnScreen, 8},
        {(unsigned int)Qt::WA_NoSystemBackground, 9},
        {(unsigned int)Qt::WA_UpdatesDisabled, 10},
        {(unsigned int)Qt::WA_Mapped, 11},
        {(unsigned int)Qt::WA_MacNoClickThrough, 12},
        {(unsigned int)Qt::WA_InputMethodEnabled, 14},
        {(unsigned int)Qt::WA_WState_Visible, 15},
        {(unsigned int)Qt::WA_WState_Hidden, 16},
        {(unsigned int)Qt::WA_ForceDisabled, 32},
        {(unsigned int)Qt::WA_KeyCompression, 33},
        {(unsigned int)Qt::WA_PendingMoveEvent, 34},
        {(unsigned int)Qt::WA_PendingResizeEvent, 35},
        {(unsigned int)Qt::WA_SetPalette, 36},
        {(unsigned int)Qt::WA_SetFont, 37},
        {(unsigned int)Qt::WA_SetCursor, 38},
        {(unsigned int)Qt::WA_NoChildEventsFromChildren, 39},
        {(unsigned int)Qt::WA_WindowModified, 41},
        {(unsigned int)Qt::WA_Resized, 42},
        {(unsigned int)Qt::WA_Moved, 43},
        {(unsigned int)Qt::WA_PendingUpdate, 44},
        {(unsigned int)Qt::WA_InvalidSize, 45},
        {(unsigned int)Qt::WA_MacBrushedMetal, 46},
        {(unsigned int)Qt::WA_MacMetalStyle, 46},
        {(unsigned int)Qt::WA_CustomWhatsThis, 47},
        {(unsigned int)Qt::WA_LayoutOnEntireRect, 48},
        {(unsigned int)Qt::WA_OutsideWSRange, 49},
        {(unsigned int)Qt::WA_GrabbedShortcut, 50},
        {(unsigned int)Qt::WA_TransparentForMouseEvents, 51},
        {(unsigned int)Qt::WA_PaintUnclipped, 52},
        {(unsigned int)Qt::WA_SetWindowIcon, 53},
        {(unsigned int)Qt::WA_NoMouseReplay, 54},
        {(unsigned int)Qt::WA_DeleteOnClose, 55},
        {(unsigned int)Qt::WA_RightToLeft, 56},
        {(unsigned int)Qt::WA_SetLayoutDirection, 57},
        {(unsigned int)Qt::WA_NoChildEventsForParent, 58},
        {(unsigned int)Qt::WA_ForceUpdatesDisabled, 59},
        {(unsigned int)Qt::WA_WState_Created, 60},
        {(unsigned int)Qt::WA_WState_CompressKeys, 61},
        {(unsigned int)Qt::WA_WState_InPaintEvent, 62},
        {(unsigned int)Qt::WA_WState_Reparented, 63},
        {(unsigned int)Qt::WA_WState_ConfigPending, 64},
        {(unsigned int)Qt::WA_WState_Polished, 66},
        {(unsigned int)Qt::WA_WState_DND, 67},
        {(unsigned int)Qt::WA_WState_OwnSizePolicy, 68},
        {(unsigned int)Qt::WA_WState_ExplicitShowHide, 69},
        {(unsigned int)Qt::WA_ShowModal, 70},
        {(unsigned int)Qt::WA_MouseNoMask, 71},
        {(unsigned int)Qt::WA_GroupLeader, 72},
        {(unsigned int)Qt::WA_NoMousePropagation, 73},
        {(unsigned int)Qt::WA_Hover, 74},
        {(unsigned int)Qt::WA_InputMethodTransparent, 75},
        {(unsigned int)Qt::WA_QuitOnClose, 76},
        {(unsigned int)Qt::WA_KeyboardFocusChange, 77},
        {(unsigned int)Qt::WA_AcceptDrops, 78},
        {(unsigned int)Qt::WA_DropSiteRegistered, 79},
        {(unsigned int)Qt::WA_ForceAcceptDrops, 79},
        {(unsigned int)Qt::WA_WindowPropagation, 80},
        {(unsigned int)Qt::WA_NoX11EventCompression, 81},
        {(unsigned int)Qt::WA_TintedBackground, 82},
        {(unsigned int)Qt::WA_X11OpenGLOverlay, 83},
        {(unsigned int)Qt::WA_AlwaysShowToolTips, 84},
        {(unsigned int)Qt::WA_MacOpaqueSizeGrip, 85},
        {(unsigned int)Qt::WA_SetStyle, 86},
        {(unsigned int)Qt::WA_SetLocale, 87},
        {(unsigned int)Qt::WA_MacShowFocusRect, 88},
        {(unsigned int)Qt::WA_MacNormalSize, 89},
        {(unsigned int)Qt::WA_MacSmallSize, 90},
        {(unsigned int)Qt::WA_MacMiniSize, 91},
        {(unsigned int)Qt::WA_LayoutUsesWidgetRect, 92},
        {(unsigned int)Qt::WA_StyledBackground, 93},
        {(unsigned int)Qt::WA_MSWindowsUseDirect3D, 94},
        {(unsigned int)Qt::WA_CanHostQMdiSubWindowTitleBar, 95},
        {(unsigned int)Qt::WA_MacAlwaysShowToolWindow, 96},
        {(unsigned int)Qt::WA_StyleSheet, 97},
        {(unsigned int)Qt::WA_ShowWithoutActivating, 98},
        {(unsigned int)Qt::WA_X11BypassTransientForHint, 99},
        {(unsigned int)Qt::WA_NativeWindow, 100},
        {(unsigned int)Qt::WA_DontCreateNativeAncestors, 101},
        {(unsigned int)Qt::WA_MacVariableSize, 102},
        {(unsigned int)Qt::WA_DontShowOnScreen, 103},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeDesktop, 104},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeDock, 105},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeToolBar, 106},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeMenu, 107},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeUtility, 108},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeSplash, 109},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeDialog, 110},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeDropDownMenu, 111},
        {(unsigned int)Qt::WA_X11NetWmWindowTypePopupMenu, 112},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeToolTip, 113},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeNotification, 114},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeCombo, 115},
        {(unsigned int)Qt::WA_X11NetWmWindowTypeDND, 116},
        {(unsigned int)Qt::WA_MacFrameworkScaled, 117},
        {(unsigned int)Qt::WA_SetWindowModality, 118},
        {(unsigned int)Qt::WA_WState_WindowOpacitySet, 119},
        {(unsigned int)Qt::WA_TranslucentBackground, 120},
        {(unsigned int)Qt::WA_AcceptTouchEvents, 121},
        {(unsigned int)Qt::WA_WState_AcceptedTouchBeginEvent, 122},
        {(unsigned int)Qt::WA_TouchPadAcceptSingleTouchEvents, 123},
        {(unsigned int)Qt::WA_X11DoNotAcceptFocus, 126},
        {(unsigned int)Qt::WA_MacNoShadow, 127},
        {(unsigned int)Qt::WA_AlwaysStackOnTop, 128},
        {(unsigned int)Qt::WA_TabletTracking, 129},
        {(unsigned int)Qt::WA_ContentsMarginsRespectsSafeArea, 130},
        {(unsigned int)Qt::WA_AttributeCount, 131},
    };

    for (int i = 0; i < 114; ++i) {
        s_widget_attribute_lookup[widget_attribute_vals[i].key] =
            (unsigned int)widget_attribute_vals[i].val;
    }

    static KeyVal window_frame_section_vals[] = {
        {(unsigned int)Qt::NoSection, 0},
        {(unsigned int)Qt::LeftSection, 1},
        {(unsigned int)Qt::TopLeftSection, 2},
        {(unsigned int)Qt::TopSection, 3},
        {(unsigned int)Qt::TopRightSection, 4},
        {(unsigned int)Qt::RightSection, 5},
        {(unsigned int)Qt::BottomRightSection, 6},
        {(unsigned int)Qt::BottomSection, 7},
        {(unsigned int)Qt::BottomLeftSection, 8},
        {(unsigned int)Qt::TitleBarArea, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_window_frame_section_lookup[window_frame_section_vals[i].key] =
            (unsigned int)window_frame_section_vals[i].val;
    }

    static KeyVal window_modality_vals[] = {
        {(unsigned int)Qt::NonModal, 0},
        {(unsigned int)Qt::WindowModal, 1},
        {(unsigned int)Qt::ApplicationModal, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_window_modality_lookup[window_modality_vals[i].key] =
            (unsigned int)window_modality_vals[i].val;
    }

    static KeyVal window_state_vals[] = {
        {(unsigned int)Qt::WindowNoState, 0},
        {(unsigned int)Qt::WindowMinimized, 1},
        {(unsigned int)Qt::WindowMaximized, 2},
        {(unsigned int)Qt::WindowFullScreen, 4},
        {(unsigned int)Qt::WindowActive, 8},
    };

    for (int i = 0; i < 5; ++i) {
        s_window_state_lookup[window_state_vals[i].key] =
            (unsigned int)window_state_vals[i].val;
    }

    static KeyVal window_type_vals[] = {
        {(unsigned int)Qt::Widget, 0},
        {(unsigned int)Qt::Window, 1},
        {(unsigned int)Qt::Dialog, 3},
        {(unsigned int)Qt::Sheet, 5},
        {(unsigned int)Qt::Drawer, 7},
        {(unsigned int)Qt::Popup, 9},
        {(unsigned int)Qt::Tool, 11},
        {(unsigned int)Qt::ToolTip, 13},
        {(unsigned int)Qt::SplashScreen, 15},
        {(unsigned int)Qt::Desktop, 17},
        {(unsigned int)Qt::SubWindow, 18},
        {(unsigned int)Qt::ForeignWindow, 33},
        {(unsigned int)Qt::CoverWindow, 65},
        {(unsigned int)Qt::WindowType_Mask, 255},
        {(unsigned int)Qt::MSWindowsFixedSizeDialogHint, 256},
        {(unsigned int)Qt::MSWindowsOwnDC, 512},
        {(unsigned int)Qt::BypassWindowManagerHint, 1024},
        {(unsigned int)Qt::X11BypassWindowManagerHint, 1024},
        {(unsigned int)Qt::FramelessWindowHint, 2048},
        {(unsigned int)Qt::WindowTitleHint, 4096},
        {(unsigned int)Qt::WindowSystemMenuHint, 8192},
        {(unsigned int)Qt::WindowMinimizeButtonHint, 16384},
        {(unsigned int)Qt::WindowMaximizeButtonHint, 32768},
        {(unsigned int)Qt::WindowMinMaxButtonsHint, 49152},
        {(unsigned int)Qt::WindowContextHelpButtonHint, 65536},
        {(unsigned int)Qt::WindowShadeButtonHint, 131072},
        {(unsigned int)Qt::WindowStaysOnTopHint, 262144},
        {(unsigned int)Qt::WindowTransparentForInput, 524288},
        {(unsigned int)Qt::WindowOverridesSystemGestures, 1048576},
        {(unsigned int)Qt::WindowDoesNotAcceptFocus, 2097152},
        {(unsigned int)Qt::MaximizeUsingFullscreenGeometryHint, 4194304},
        {(unsigned int)Qt::CustomizeWindowHint, 33554432},
        {(unsigned int)Qt::WindowStaysOnBottomHint, 67108864},
        {(unsigned int)Qt::WindowCloseButtonHint, 134217728},
        {(unsigned int)Qt::MacWindowToolBarButtonHint, 268435456},
        {(unsigned int)Qt::BypassGraphicsProxyWidget, 536870912},
        {(unsigned int)Qt::NoDropShadowWindowHint, 1073741824},
        {(unsigned int)Qt::WindowFullscreenButtonHint, 2147483648},
    };

    for (int i = 0; i < 38; ++i) {
        s_window_type_lookup[window_type_vals[i].key] =
            (unsigned int)window_type_vals[i].val;
    }

    static KeyVal yuv_layout_vals[] = {
        {(unsigned int)QPixelFormat::YUV444, 0},
        {(unsigned int)QPixelFormat::YUV422, 1},
        {(unsigned int)QPixelFormat::YUV411, 2},
        {(unsigned int)QPixelFormat::YUV420P, 3},
        {(unsigned int)QPixelFormat::YUV420SP, 4},
        {(unsigned int)QPixelFormat::YV12, 5},
        {(unsigned int)QPixelFormat::UYVY, 6},
        {(unsigned int)QPixelFormat::YUYV, 7},
        {(unsigned int)QPixelFormat::NV12, 8},
        {(unsigned int)QPixelFormat::NV21, 9},
        {(unsigned int)QPixelFormat::IMC1, 10},
        {(unsigned int)QPixelFormat::IMC2, 11},
        {(unsigned int)QPixelFormat::IMC3, 12},
        {(unsigned int)QPixelFormat::IMC4, 13},
        {(unsigned int)QPixelFormat::Y8, 14},
        {(unsigned int)QPixelFormat::Y16, 15},
    };

    for (int i = 0; i < 16; ++i) {
        s_yuv_layout_lookup[yuv_layout_vals[i].key] =
            (unsigned int)yuv_layout_vals[i].val;
    }
}
