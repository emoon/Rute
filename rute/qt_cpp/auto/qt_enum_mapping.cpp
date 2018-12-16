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
    int val, key;
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
std::map<int, int> s_resolve_properties_lookup;
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
        {(int)QLineEdit::LeadingPosition, 0},
        {(int)QLineEdit::TrailingPosition, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_action_position_lookup[action_position_vals[i].key] =
            action_position_vals[i].val;
    }

    static KeyVal alignment_flag_vals[] = {
        {(int)Qt::AlignLeft, 0},
        {(int)Qt::AlignLeading, 1},
        {(int)Qt::AlignRight, 2},
        {(int)Qt::AlignTrailing, 3},
        {(int)Qt::AlignHCenter, 4},
        {(int)Qt::AlignJustify, 5},
        {(int)Qt::AlignAbsolute, 6},
        {(int)Qt::AlignHorizontal_Mask, 7},
        {(int)Qt::AlignTop, 8},
        {(int)Qt::AlignBottom, 9},
        {(int)Qt::AlignVCenter, 10},
        {(int)Qt::AlignBaseline, 11},
        {(int)Qt::AlignVertical_Mask, 12},
        {(int)Qt::AlignCenter, 13},
    };

    for (int i = 0; i < 14; ++i) {
        s_alignment_flag_lookup[alignment_flag_vals[i].key] =
            alignment_flag_vals[i].val;
    }

    static KeyVal alpha_position_vals[] = {
        {(int)QPixelFormat::AtBeginning, 0},
        {(int)QPixelFormat::AtEnd, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_position_lookup[alpha_position_vals[i].key] =
            alpha_position_vals[i].val;
    }

    static KeyVal alpha_premultiplied_vals[] = {
        {(int)QPixelFormat::NotPremultiplied, 0},
        {(int)QPixelFormat::Premultiplied, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_premultiplied_lookup[alpha_premultiplied_vals[i].key] =
            alpha_premultiplied_vals[i].val;
    }

    static KeyVal alpha_usage_vals[] = {
        {(int)QPixelFormat::UsesAlpha, 0},
        {(int)QPixelFormat::IgnoresAlpha, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_alpha_usage_lookup[alpha_usage_vals[i].key] = alpha_usage_vals[i].val;
    }

    static KeyVal ancestor_mode_vals[] = {
        {(int)QWindow::ExcludeTransients, 0},
        {(int)QWindow::IncludeTransients, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_ancestor_mode_lookup[ancestor_mode_vals[i].key] =
            ancestor_mode_vals[i].val;
    }

    static KeyVal anchor_point_vals[] = {
        {(int)Qt::AnchorLeft, 0},
        {(int)Qt::AnchorHorizontalCenter, 1},
        {(int)Qt::AnchorRight, 2},
        {(int)Qt::AnchorTop, 3},
        {(int)Qt::AnchorVerticalCenter, 4},
        {(int)Qt::AnchorBottom, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_anchor_point_lookup[anchor_point_vals[i].key] =
            anchor_point_vals[i].val;
    }

    static KeyVal application_attribute_vals[] = {
        {(int)Qt::AA_ImmediateWidgetCreation, 0},
        {(int)Qt::AA_MSWindowsUseDirect3DByDefault, 1},
        {(int)Qt::AA_DontShowIconsInMenus, 2},
        {(int)Qt::AA_NativeWindows, 3},
        {(int)Qt::AA_DontCreateNativeWidgetSiblings, 4},
        {(int)Qt::AA_PluginApplication, 5},
        {(int)Qt::AA_MacPluginApplication, 6},
        {(int)Qt::AA_DontUseNativeMenuBar, 7},
        {(int)Qt::AA_Use96Dpi, 8},
        {(int)Qt::AA_X11InitThreads, 9},
        {(int)Qt::AA_SynthesizeTouchForUnhandledMouseEvents, 10},
        {(int)Qt::AA_SynthesizeMouseForUnhandledTouchEvents, 11},
        {(int)Qt::AA_UseHighDpiPixmaps, 12},
        {(int)Qt::AA_ForceRasterWidgets, 13},
        {(int)Qt::AA_UseDesktopOpenGL, 14},
        {(int)Qt::AA_UseOpenGLES, 15},
        {(int)Qt::AA_UseSoftwareOpenGL, 16},
        {(int)Qt::AA_ShareOpenGLContexts, 17},
        {(int)Qt::AA_SetPalette, 18},
        {(int)Qt::AA_EnableHighDpiScaling, 19},
        {(int)Qt::AA_DisableHighDpiScaling, 20},
        {(int)Qt::AA_UseStyleSheetPropagationInWidgetStyles, 21},
        {(int)Qt::AA_DontUseNativeDialogs, 22},
        {(int)Qt::AA_SynthesizeMouseForUnhandledTabletEvents, 23},
        {(int)Qt::AA_CompressHighFrequencyEvents, 24},
        {(int)Qt::AA_DontCheckOpenGLContextThreadAffinity, 25},
        {(int)Qt::AA_DisableShaderDiskCache, 26},
        {(int)Qt::AA_DontShowShortcutsInContextMenus, 27},
        {(int)Qt::AA_CompressTabletEvents, 28},
        {(int)Qt::AA_DisableWindowContextHelpButton, 29},
        {(int)Qt::AA_AttributeCount, 30},
    };

    for (int i = 0; i < 31; ++i) {
        s_application_attribute_lookup[application_attribute_vals[i].key] =
            application_attribute_vals[i].val;
    }

    static KeyVal application_state_vals[] = {
        {(int)Qt::ApplicationSuspended, 0},
        {(int)Qt::ApplicationHidden, 1},
        {(int)Qt::ApplicationInactive, 2},
        {(int)Qt::ApplicationActive, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_application_state_lookup[application_state_vals[i].key] =
            application_state_vals[i].val;
    }

    static KeyVal arrow_type_vals[] = {
        {(int)Qt::NoArrow, 0},    {(int)Qt::UpArrow, 1},
        {(int)Qt::DownArrow, 2},  {(int)Qt::LeftArrow, 3},
        {(int)Qt::RightArrow, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_arrow_type_lookup[arrow_type_vals[i].key] = arrow_type_vals[i].val;
    }

    static KeyVal aspect_ratio_mode_vals[] = {
        {(int)Qt::IgnoreAspectRatio, 0},
        {(int)Qt::KeepAspectRatio, 1},
        {(int)Qt::KeepAspectRatioByExpanding, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_aspect_ratio_mode_lookup[aspect_ratio_mode_vals[i].key] =
            aspect_ratio_mode_vals[i].val;
    }

    static KeyVal axis_vals[] = {
        {(int)Qt::XAxis, 0},
        {(int)Qt::YAxis, 1},
        {(int)Qt::ZAxis, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_axis_lookup[axis_vals[i].key] = axis_vals[i].val;
    }

    static KeyVal bg_mode_vals[] = {
        {(int)Qt::TransparentMode, 0},
        {(int)Qt::OpaqueMode, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_bg_mode_lookup[bg_mode_vals[i].key] = bg_mode_vals[i].val;
    }

    static KeyVal brush_style_vals[] = {
        {(int)Qt::NoBrush, 0},
        {(int)Qt::SolidPattern, 1},
        {(int)Qt::Dense1Pattern, 2},
        {(int)Qt::Dense2Pattern, 3},
        {(int)Qt::Dense3Pattern, 4},
        {(int)Qt::Dense4Pattern, 5},
        {(int)Qt::Dense5Pattern, 6},
        {(int)Qt::Dense6Pattern, 7},
        {(int)Qt::Dense7Pattern, 8},
        {(int)Qt::HorPattern, 9},
        {(int)Qt::VerPattern, 10},
        {(int)Qt::CrossPattern, 11},
        {(int)Qt::BDiagPattern, 12},
        {(int)Qt::FDiagPattern, 13},
        {(int)Qt::DiagCrossPattern, 14},
        {(int)Qt::LinearGradientPattern, 15},
        {(int)Qt::RadialGradientPattern, 16},
        {(int)Qt::ConicalGradientPattern, 17},
        {(int)Qt::TexturePattern, 18},
    };

    for (int i = 0; i < 19; ++i) {
        s_brush_style_lookup[brush_style_vals[i].key] = brush_style_vals[i].val;
    }

    static KeyVal byte_order_vals[] = {
        {(int)QPixelFormat::LittleEndian, 0},
        {(int)QPixelFormat::BigEndian, 1},
        {(int)QPixelFormat::CurrentSystemEndian, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_byte_order_lookup[byte_order_vals[i].key] = byte_order_vals[i].val;
    }

    static KeyVal capitalization_vals[] = {
        {(int)QFont::MixedCase, 0},    {(int)QFont::AllUppercase, 1},
        {(int)QFont::AllLowercase, 2}, {(int)QFont::SmallCaps, 3},
        {(int)QFont::Capitalize, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_capitalization_lookup[capitalization_vals[i].key] =
            capitalization_vals[i].val;
    }

    static KeyVal case_sensitivity_vals[] = {
        {(int)Qt::CaseInsensitive, 0},
        {(int)Qt::CaseSensitive, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_case_sensitivity_lookup[case_sensitivity_vals[i].key] =
            case_sensitivity_vals[i].val;
    }

    static KeyVal check_state_vals[] = {
        {(int)Qt::Unchecked, 0},
        {(int)Qt::PartiallyChecked, 1},
        {(int)Qt::Checked, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_check_state_lookup[check_state_vals[i].key] = check_state_vals[i].val;
    }

    static KeyVal checksum_type_vals[] = {
        {(int)Qt::ChecksumIso3309, 0},
        {(int)Qt::ChecksumItuV41, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_checksum_type_lookup[checksum_type_vals[i].key] =
            checksum_type_vals[i].val;
    }

    static KeyVal clip_operation_vals[] = {
        {(int)Qt::NoClip, 0},
        {(int)Qt::ReplaceClip, 1},
        {(int)Qt::IntersectClip, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_clip_operation_lookup[clip_operation_vals[i].key] =
            clip_operation_vals[i].val;
    }

    static KeyVal color_group_vals[] = {
        {(int)QPalette::Active, 0},   {(int)QPalette::Disabled, 1},
        {(int)QPalette::Inactive, 2}, {(int)QPalette::NColorGroups, 3},
        {(int)QPalette::Current, 4},  {(int)QPalette::All, 5},
        {(int)QPalette::Normal, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_color_group_lookup[color_group_vals[i].key] = color_group_vals[i].val;
    }

    static KeyVal color_model_vals[] = {
        {(int)QPixelFormat::RGB, 0},     {(int)QPixelFormat::BGR, 1},
        {(int)QPixelFormat::Indexed, 2}, {(int)QPixelFormat::Grayscale, 3},
        {(int)QPixelFormat::CMYK, 4},    {(int)QPixelFormat::HSL, 5},
        {(int)QPixelFormat::HSV, 6},     {(int)QPixelFormat::YUV, 7},
        {(int)QPixelFormat::Alpha, 8},
    };

    for (int i = 0; i < 9; ++i) {
        s_color_model_lookup[color_model_vals[i].key] = color_model_vals[i].val;
    }

    static KeyVal color_role_vals[] = {
        {(int)QPalette::WindowText, 0},
        {(int)QPalette::Button, 1},
        {(int)QPalette::Light, 2},
        {(int)QPalette::Midlight, 3},
        {(int)QPalette::Dark, 4},
        {(int)QPalette::Mid, 5},
        {(int)QPalette::Text, 6},
        {(int)QPalette::BrightText, 7},
        {(int)QPalette::ButtonText, 8},
        {(int)QPalette::Base, 9},
        {(int)QPalette::Window, 10},
        {(int)QPalette::Shadow, 11},
        {(int)QPalette::Highlight, 12},
        {(int)QPalette::HighlightedText, 13},
        {(int)QPalette::Link, 14},
        {(int)QPalette::LinkVisited, 15},
        {(int)QPalette::AlternateBase, 16},
        {(int)QPalette::NoRole, 17},
        {(int)QPalette::ToolTipBase, 18},
        {(int)QPalette::ToolTipText, 19},
        {(int)QPalette::NColorRoles, 20},
        {(int)QPalette::Foreground, 21},
        {(int)QPalette::Background, 22},
    };

    for (int i = 0; i < 23; ++i) {
        s_color_role_lookup[color_role_vals[i].key] = color_role_vals[i].val;
    }

    static KeyVal color_space_vals[] = {
        {(int)QSurfaceFormat::DefaultColorSpace, 0},
        {(int)QSurfaceFormat::sRGBColorSpace, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_color_space_lookup[color_space_vals[i].key] = color_space_vals[i].val;
    }

    static KeyVal color_spec_vals[] = {
        {(int)QApplication::NormalColor, 0},
        {(int)QApplication::CustomColor, 1},
        {(int)QApplication::ManyColor, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_color_spec_lookup[color_spec_vals[i].key] = color_spec_vals[i].val;
    }

    static KeyVal complex_control_vals[] = {
        {(int)QStyle::CC_SpinBox, 0},     {(int)QStyle::CC_ComboBox, 1},
        {(int)QStyle::CC_ScrollBar, 2},   {(int)QStyle::CC_Slider, 3},
        {(int)QStyle::CC_ToolButton, 4},  {(int)QStyle::CC_TitleBar, 5},
        {(int)QStyle::CC_Dial, 6},        {(int)QStyle::CC_GroupBox, 7},
        {(int)QStyle::CC_MdiControls, 8}, {(int)QStyle::CC_CustomBase, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_complex_control_lookup[complex_control_vals[i].key] =
            complex_control_vals[i].val;
    }

    static KeyVal composition_mode_vals[] = {
        {(int)QPainter::CompositionMode_SourceOver, 0},
        {(int)QPainter::CompositionMode_DestinationOver, 1},
        {(int)QPainter::CompositionMode_Clear, 2},
        {(int)QPainter::CompositionMode_Source, 3},
        {(int)QPainter::CompositionMode_Destination, 4},
        {(int)QPainter::CompositionMode_SourceIn, 5},
        {(int)QPainter::CompositionMode_DestinationIn, 6},
        {(int)QPainter::CompositionMode_SourceOut, 7},
        {(int)QPainter::CompositionMode_DestinationOut, 8},
        {(int)QPainter::CompositionMode_SourceAtop, 9},
        {(int)QPainter::CompositionMode_DestinationAtop, 10},
        {(int)QPainter::CompositionMode_Xor, 11},
        {(int)QPainter::CompositionMode_Plus, 12},
        {(int)QPainter::CompositionMode_Multiply, 13},
        {(int)QPainter::CompositionMode_Screen, 14},
        {(int)QPainter::CompositionMode_Overlay, 15},
        {(int)QPainter::CompositionMode_Darken, 16},
        {(int)QPainter::CompositionMode_Lighten, 17},
        {(int)QPainter::CompositionMode_ColorDodge, 18},
        {(int)QPainter::CompositionMode_ColorBurn, 19},
        {(int)QPainter::CompositionMode_HardLight, 20},
        {(int)QPainter::CompositionMode_SoftLight, 21},
        {(int)QPainter::CompositionMode_Difference, 22},
        {(int)QPainter::CompositionMode_Exclusion, 23},
        {(int)QPainter::RasterOp_SourceOrDestination, 24},
        {(int)QPainter::RasterOp_SourceAndDestination, 25},
        {(int)QPainter::RasterOp_SourceXorDestination, 26},
        {(int)QPainter::RasterOp_NotSourceAndNotDestination, 27},
        {(int)QPainter::RasterOp_NotSourceOrNotDestination, 28},
        {(int)QPainter::RasterOp_NotSourceXorDestination, 29},
        {(int)QPainter::RasterOp_NotSource, 30},
        {(int)QPainter::RasterOp_NotSourceAndDestination, 31},
        {(int)QPainter::RasterOp_SourceAndNotDestination, 32},
        {(int)QPainter::RasterOp_NotSourceOrDestination, 33},
        {(int)QPainter::RasterOp_SourceOrNotDestination, 34},
        {(int)QPainter::RasterOp_ClearDestination, 35},
        {(int)QPainter::RasterOp_SetDestination, 36},
        {(int)QPainter::RasterOp_NotDestination, 37},
    };

    for (int i = 0; i < 38; ++i) {
        s_composition_mode_lookup[composition_mode_vals[i].key] =
            composition_mode_vals[i].val;
    }

    static KeyVal connection_type_vals[] = {
        {(int)Qt::AutoConnection, 0},   {(int)Qt::DirectConnection, 1},
        {(int)Qt::QueuedConnection, 2}, {(int)Qt::BlockingQueuedConnection, 3},
        {(int)Qt::UniqueConnection, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_connection_type_lookup[connection_type_vals[i].key] =
            connection_type_vals[i].val;
    }

    static KeyVal contents_type_vals[] = {
        {(int)QStyle::CT_PushButton, 0},
        {(int)QStyle::CT_CheckBox, 1},
        {(int)QStyle::CT_RadioButton, 2},
        {(int)QStyle::CT_ToolButton, 3},
        {(int)QStyle::CT_ComboBox, 4},
        {(int)QStyle::CT_Splitter, 5},
        {(int)QStyle::CT_ProgressBar, 6},
        {(int)QStyle::CT_MenuItem, 7},
        {(int)QStyle::CT_MenuBarItem, 8},
        {(int)QStyle::CT_MenuBar, 9},
        {(int)QStyle::CT_Menu, 10},
        {(int)QStyle::CT_TabBarTab, 11},
        {(int)QStyle::CT_Slider, 12},
        {(int)QStyle::CT_ScrollBar, 13},
        {(int)QStyle::CT_LineEdit, 14},
        {(int)QStyle::CT_SpinBox, 15},
        {(int)QStyle::CT_SizeGrip, 16},
        {(int)QStyle::CT_TabWidget, 17},
        {(int)QStyle::CT_DialogButtons, 18},
        {(int)QStyle::CT_HeaderSection, 19},
        {(int)QStyle::CT_GroupBox, 20},
        {(int)QStyle::CT_MdiControls, 21},
        {(int)QStyle::CT_ItemViewItem, 22},
        {(int)QStyle::CT_CustomBase, 23},
    };

    for (int i = 0; i < 24; ++i) {
        s_contents_type_lookup[contents_type_vals[i].key] =
            contents_type_vals[i].val;
    }

    static KeyVal context_menu_policy_vals[] = {
        {(int)Qt::NoContextMenu, 0},      {(int)Qt::DefaultContextMenu, 1},
        {(int)Qt::ActionsContextMenu, 2}, {(int)Qt::CustomContextMenu, 3},
        {(int)Qt::PreventContextMenu, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_context_menu_policy_lookup[context_menu_policy_vals[i].key] =
            context_menu_policy_vals[i].val;
    }

    static KeyVal control_element_vals[] = {
        {(int)QStyle::CE_PushButton, 0},
        {(int)QStyle::CE_PushButtonBevel, 1},
        {(int)QStyle::CE_PushButtonLabel, 2},
        {(int)QStyle::CE_CheckBox, 3},
        {(int)QStyle::CE_CheckBoxLabel, 4},
        {(int)QStyle::CE_RadioButton, 5},
        {(int)QStyle::CE_RadioButtonLabel, 6},
        {(int)QStyle::CE_TabBarTab, 7},
        {(int)QStyle::CE_TabBarTabShape, 8},
        {(int)QStyle::CE_TabBarTabLabel, 9},
        {(int)QStyle::CE_ProgressBar, 10},
        {(int)QStyle::CE_ProgressBarGroove, 11},
        {(int)QStyle::CE_ProgressBarContents, 12},
        {(int)QStyle::CE_ProgressBarLabel, 13},
        {(int)QStyle::CE_MenuItem, 14},
        {(int)QStyle::CE_MenuScroller, 15},
        {(int)QStyle::CE_MenuVMargin, 16},
        {(int)QStyle::CE_MenuHMargin, 17},
        {(int)QStyle::CE_MenuTearoff, 18},
        {(int)QStyle::CE_MenuEmptyArea, 19},
        {(int)QStyle::CE_MenuBarItem, 20},
        {(int)QStyle::CE_MenuBarEmptyArea, 21},
        {(int)QStyle::CE_ToolButtonLabel, 22},
        {(int)QStyle::CE_Header, 23},
        {(int)QStyle::CE_HeaderSection, 24},
        {(int)QStyle::CE_HeaderLabel, 25},
        {(int)QStyle::CE_ToolBoxTab, 26},
        {(int)QStyle::CE_SizeGrip, 27},
        {(int)QStyle::CE_Splitter, 28},
        {(int)QStyle::CE_RubberBand, 29},
        {(int)QStyle::CE_DockWidgetTitle, 30},
        {(int)QStyle::CE_ScrollBarAddLine, 31},
        {(int)QStyle::CE_ScrollBarSubLine, 32},
        {(int)QStyle::CE_ScrollBarAddPage, 33},
        {(int)QStyle::CE_ScrollBarSubPage, 34},
        {(int)QStyle::CE_ScrollBarSlider, 35},
        {(int)QStyle::CE_ScrollBarFirst, 36},
        {(int)QStyle::CE_ScrollBarLast, 37},
        {(int)QStyle::CE_FocusFrame, 38},
        {(int)QStyle::CE_ComboBoxLabel, 39},
        {(int)QStyle::CE_ToolBar, 40},
        {(int)QStyle::CE_ToolBoxTabShape, 41},
        {(int)QStyle::CE_ToolBoxTabLabel, 42},
        {(int)QStyle::CE_HeaderEmptyArea, 43},
        {(int)QStyle::CE_ColumnViewGrip, 44},
        {(int)QStyle::CE_ItemViewItem, 45},
        {(int)QStyle::CE_ShapedFrame, 46},
        {(int)QStyle::CE_CustomBase, 47},
    };

    for (int i = 0; i < 48; ++i) {
        s_control_element_lookup[control_element_vals[i].key] =
            control_element_vals[i].val;
    }

    static KeyVal control_type_vals[] = {
        {(int)QSizePolicy::DefaultType, 0},  {(int)QSizePolicy::ButtonBox, 1},
        {(int)QSizePolicy::CheckBox, 2},     {(int)QSizePolicy::ComboBox, 3},
        {(int)QSizePolicy::Frame, 4},        {(int)QSizePolicy::GroupBox, 5},
        {(int)QSizePolicy::Label, 6},        {(int)QSizePolicy::Line, 7},
        {(int)QSizePolicy::LineEdit, 8},     {(int)QSizePolicy::PushButton, 9},
        {(int)QSizePolicy::RadioButton, 10}, {(int)QSizePolicy::Slider, 11},
        {(int)QSizePolicy::SpinBox, 12},     {(int)QSizePolicy::TabWidget, 13},
        {(int)QSizePolicy::ToolButton, 14},
    };

    for (int i = 0; i < 15; ++i) {
        s_control_type_lookup[control_type_vals[i].key] =
            control_type_vals[i].val;
    }

    static KeyVal coordinate_mode_vals[] = {
        {(int)QGradient::LogicalMode, 0},
        {(int)QGradient::StretchToDeviceMode, 1},
        {(int)QGradient::ObjectBoundingMode, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_coordinate_mode_lookup[coordinate_mode_vals[i].key] =
            coordinate_mode_vals[i].val;
    }

    static KeyVal coordinate_system_vals[] = {
        {(int)Qt::DeviceCoordinates, 0},
        {(int)Qt::LogicalCoordinates, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_coordinate_system_lookup[coordinate_system_vals[i].key] =
            coordinate_system_vals[i].val;
    }

    static KeyVal core_application_fix_me_enums_vals[] = {
        {(int)QCoreApplication::ApplicationFlags, 330498},
    };

    for (int i = 0; i < 1; ++i) {
        s_core_application_fix_me_enums_lookup
            [core_application_fix_me_enums_vals[i].key] =
                core_application_fix_me_enums_vals[i].val;
    }

    static KeyVal corner_vals[] = {
        {(int)Qt::TopLeftCorner, 0},
        {(int)Qt::TopRightCorner, 1},
        {(int)Qt::BottomLeftCorner, 2},
        {(int)Qt::BottomRightCorner, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_corner_lookup[corner_vals[i].key] = corner_vals[i].val;
    }

    static KeyVal cursor_move_style_vals[] = {
        {(int)Qt::LogicalMoveStyle, 0},
        {(int)Qt::VisualMoveStyle, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_cursor_move_style_lookup[cursor_move_style_vals[i].key] =
            cursor_move_style_vals[i].val;
    }

    static KeyVal cursor_shape_vals[] = {
        {(int)Qt::ArrowCursor, 0},       {(int)Qt::UpArrowCursor, 1},
        {(int)Qt::CrossCursor, 2},       {(int)Qt::WaitCursor, 3},
        {(int)Qt::IBeamCursor, 4},       {(int)Qt::SizeVerCursor, 5},
        {(int)Qt::SizeHorCursor, 6},     {(int)Qt::SizeBDiagCursor, 7},
        {(int)Qt::SizeFDiagCursor, 8},   {(int)Qt::SizeAllCursor, 9},
        {(int)Qt::BlankCursor, 10},      {(int)Qt::SplitVCursor, 11},
        {(int)Qt::SplitHCursor, 12},     {(int)Qt::PointingHandCursor, 13},
        {(int)Qt::ForbiddenCursor, 14},  {(int)Qt::WhatsThisCursor, 15},
        {(int)Qt::BusyCursor, 16},       {(int)Qt::OpenHandCursor, 17},
        {(int)Qt::ClosedHandCursor, 18}, {(int)Qt::DragCopyCursor, 19},
        {(int)Qt::DragMoveCursor, 20},   {(int)Qt::DragLinkCursor, 21},
        {(int)Qt::LastCursor, 22},       {(int)Qt::BitmapCursor, 23},
        {(int)Qt::CustomCursor, 24},
    };

    for (int i = 0; i < 25; ++i) {
        s_cursor_shape_lookup[cursor_shape_vals[i].key] =
            cursor_shape_vals[i].val;
    }

    static KeyVal date_format_vals[] = {
        {(int)Qt::TextDate, 0},
        {(int)Qt::ISODate, 1},
        {(int)Qt::SystemLocaleDate, 2},
        {(int)Qt::LocalDate, 3},
        {(int)Qt::LocaleDate, 4},
        {(int)Qt::SystemLocaleShortDate, 5},
        {(int)Qt::SystemLocaleLongDate, 6},
        {(int)Qt::DefaultLocaleShortDate, 7},
        {(int)Qt::DefaultLocaleLongDate, 8},
        {(int)Qt::RFC2822Date, 9},
        {(int)Qt::ISODateWithMs, 10},
    };

    for (int i = 0; i < 11; ++i) {
        s_date_format_lookup[date_format_vals[i].key] = date_format_vals[i].val;
    }

    static KeyVal day_of_week_vals[] = {
        {(int)Qt::Monday, 0},   {(int)Qt::Tuesday, 1}, {(int)Qt::Wednesday, 2},
        {(int)Qt::Thursday, 3}, {(int)Qt::Friday, 4},  {(int)Qt::Saturday, 5},
        {(int)Qt::Sunday, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_day_of_week_lookup[day_of_week_vals[i].key] = day_of_week_vals[i].val;
    }

    static KeyVal dirty_flag_vals[] = {
        {(int)QPaintEngine::DirtyPen, 0},
        {(int)QPaintEngine::DirtyBrush, 1},
        {(int)QPaintEngine::DirtyBrushOrigin, 2},
        {(int)QPaintEngine::DirtyFont, 3},
        {(int)QPaintEngine::DirtyBackground, 4},
        {(int)QPaintEngine::DirtyBackgroundMode, 5},
        {(int)QPaintEngine::DirtyTransform, 6},
        {(int)QPaintEngine::DirtyClipRegion, 7},
        {(int)QPaintEngine::DirtyClipPath, 8},
        {(int)QPaintEngine::DirtyHints, 9},
        {(int)QPaintEngine::DirtyCompositionMode, 10},
        {(int)QPaintEngine::DirtyClipEnabled, 11},
        {(int)QPaintEngine::DirtyOpacity, 12},
        {(int)QPaintEngine::AllDirty, 13},
    };

    for (int i = 0; i < 14; ++i) {
        s_dirty_flag_lookup[dirty_flag_vals[i].key] = dirty_flag_vals[i].val;
    }

    static KeyVal dock_widget_area_vals[] = {
        {(int)Qt::LeftDockWidgetArea, 0},  {(int)Qt::RightDockWidgetArea, 1},
        {(int)Qt::TopDockWidgetArea, 2},   {(int)Qt::BottomDockWidgetArea, 3},
        {(int)Qt::DockWidgetArea_Mask, 4}, {(int)Qt::AllDockWidgetAreas, 5},
        {(int)Qt::NoDockWidgetArea, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_dock_widget_area_lookup[dock_widget_area_vals[i].key] =
            dock_widget_area_vals[i].val;
    }

    static KeyVal dock_widget_area_sizes_vals[] = {
        {(int)Qt::NDockWidgetAreas, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_dock_widget_area_sizes_lookup[dock_widget_area_sizes_vals[i].key] =
            dock_widget_area_sizes_vals[i].val;
    }

    static KeyVal drop_action_vals[] = {
        {(int)Qt::CopyAction, 0},       {(int)Qt::MoveAction, 1},
        {(int)Qt::LinkAction, 2},       {(int)Qt::ActionMask, 3},
        {(int)Qt::TargetMoveAction, 4}, {(int)Qt::IgnoreAction, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_drop_action_lookup[drop_action_vals[i].key] = drop_action_vals[i].val;
    }

    static KeyVal echo_mode_vals[] = {
        {(int)QLineEdit::Normal, 0},
        {(int)QLineEdit::NoEcho, 1},
        {(int)QLineEdit::Password, 2},
        {(int)QLineEdit::PasswordEchoOnEdit, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_echo_mode_lookup[echo_mode_vals[i].key] = echo_mode_vals[i].val;
    }

    static KeyVal edge_vals[] = {
        {(int)Qt::TopEdge, 0},
        {(int)Qt::LeftEdge, 1},
        {(int)Qt::RightEdge, 2},
        {(int)Qt::BottomEdge, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_edge_lookup[edge_vals[i].key] = edge_vals[i].val;
    }

    static KeyVal enter_key_type_vals[] = {
        {(int)Qt::EnterKeyDefault, 0}, {(int)Qt::EnterKeyReturn, 1},
        {(int)Qt::EnterKeyDone, 2},    {(int)Qt::EnterKeyGo, 3},
        {(int)Qt::EnterKeySend, 4},    {(int)Qt::EnterKeySearch, 5},
        {(int)Qt::EnterKeyNext, 6},    {(int)Qt::EnterKeyPrevious, 7},
    };

    for (int i = 0; i < 8; ++i) {
        s_enter_key_type_lookup[enter_key_type_vals[i].key] =
            enter_key_type_vals[i].val;
    }

    static KeyVal event_priority_vals[] = {
        {(int)Qt::HighEventPriority, 0},
        {(int)Qt::NormalEventPriority, 1},
        {(int)Qt::LowEventPriority, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_event_priority_lookup[event_priority_vals[i].key] =
            event_priority_vals[i].val;
    }

    static KeyVal fill_rule_vals[] = {
        {(int)Qt::OddEvenFill, 0},
        {(int)Qt::WindingFill, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_fill_rule_lookup[fill_rule_vals[i].key] = fill_rule_vals[i].val;
    }

    static KeyVal find_child_option_vals[] = {
        {(int)Qt::FindDirectChildrenOnly, 0},
        {(int)Qt::FindChildrenRecursively, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_find_child_option_lookup[find_child_option_vals[i].key] =
            find_child_option_vals[i].val;
    }

    static KeyVal focus_policy_vals[] = {
        {(int)Qt::NoFocus, 0},    {(int)Qt::TabFocus, 1},
        {(int)Qt::ClickFocus, 2}, {(int)Qt::StrongFocus, 3},
        {(int)Qt::WheelFocus, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_focus_policy_lookup[focus_policy_vals[i].key] =
            focus_policy_vals[i].val;
    }

    static KeyVal focus_reason_vals[] = {
        {(int)Qt::MouseFocusReason, 0},   {(int)Qt::TabFocusReason, 1},
        {(int)Qt::BacktabFocusReason, 2}, {(int)Qt::ActiveWindowFocusReason, 3},
        {(int)Qt::PopupFocusReason, 4},   {(int)Qt::ShortcutFocusReason, 5},
        {(int)Qt::MenuBarFocusReason, 6}, {(int)Qt::OtherFocusReason, 7},
        {(int)Qt::NoFocusReason, 8},
    };

    for (int i = 0; i < 9; ++i) {
        s_focus_reason_lookup[focus_reason_vals[i].key] =
            focus_reason_vals[i].val;
    }

    static KeyVal format_vals[] = {
        {(int)QImage::Format_Invalid, 0},
        {(int)QImage::Format_Mono, 1},
        {(int)QImage::Format_MonoLSB, 2},
        {(int)QImage::Format_Indexed8, 3},
        {(int)QImage::Format_RGB32, 4},
        {(int)QImage::Format_ARGB32, 5},
        {(int)QImage::Format_ARGB32_Premultiplied, 6},
        {(int)QImage::Format_RGB16, 7},
        {(int)QImage::Format_ARGB8565_Premultiplied, 8},
        {(int)QImage::Format_RGB666, 9},
        {(int)QImage::Format_ARGB6666_Premultiplied, 10},
        {(int)QImage::Format_RGB555, 11},
        {(int)QImage::Format_ARGB8555_Premultiplied, 12},
        {(int)QImage::Format_RGB888, 13},
        {(int)QImage::Format_RGB444, 14},
        {(int)QImage::Format_ARGB4444_Premultiplied, 15},
        {(int)QImage::Format_RGBX8888, 16},
        {(int)QImage::Format_RGBA8888, 17},
        {(int)QImage::Format_RGBA8888_Premultiplied, 18},
        {(int)QImage::Format_BGR30, 19},
        {(int)QImage::Format_A2BGR30_Premultiplied, 20},
        {(int)QImage::Format_RGB30, 21},
        {(int)QImage::Format_A2RGB30_Premultiplied, 22},
        {(int)QImage::Format_Alpha8, 23},
        {(int)QImage::Format_Grayscale8, 24},
        {(int)QImage::NImageFormats, 25},
    };

    for (int i = 0; i < 26; ++i) {
        s_format_lookup[format_vals[i].key] = format_vals[i].val;
    }

    static KeyVal format_option_vals[] = {
        {(int)QSurfaceFormat::StereoBuffers, 0},
        {(int)QSurfaceFormat::DebugContext, 1},
        {(int)QSurfaceFormat::DeprecatedFunctions, 2},
        {(int)QSurfaceFormat::ResetNotification, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_format_option_lookup[format_option_vals[i].key] =
            format_option_vals[i].val;
    }

    static KeyVal gesture_flag_vals[] = {
        {(int)Qt::DontStartGestureOnChildren, 0},
        {(int)Qt::ReceivePartialGestures, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_gesture_flag_lookup[gesture_flag_vals[i].key] =
            gesture_flag_vals[i].val;
    }

    static KeyVal gesture_state_vals[] = {
        {(int)Qt::NoGesture, 0},       {(int)Qt::GestureStarted, 1},
        {(int)Qt::GestureUpdated, 2},  {(int)Qt::GestureFinished, 3},
        {(int)Qt::GestureCanceled, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_gesture_state_lookup[gesture_state_vals[i].key] =
            gesture_state_vals[i].val;
    }

    static KeyVal gesture_type_vals[] = {
        {(int)Qt::TapGesture, 0},      {(int)Qt::TapAndHoldGesture, 1},
        {(int)Qt::PanGesture, 2},      {(int)Qt::PinchGesture, 3},
        {(int)Qt::SwipeGesture, 4},    {(int)Qt::CustomGesture, 5},
        {(int)Qt::LastGestureType, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_gesture_type_lookup[gesture_type_vals[i].key] =
            gesture_type_vals[i].val;
    }

    static KeyVal global_color_vals[] = {
        {(int)Qt::color0, 0},      {(int)Qt::color1, 1},
        {(int)Qt::black, 2},       {(int)Qt::white, 3},
        {(int)Qt::darkGray, 4},    {(int)Qt::gray, 5},
        {(int)Qt::lightGray, 6},   {(int)Qt::red, 7},
        {(int)Qt::green, 8},       {(int)Qt::blue, 9},
        {(int)Qt::cyan, 10},       {(int)Qt::magenta, 11},
        {(int)Qt::yellow, 12},     {(int)Qt::darkRed, 13},
        {(int)Qt::darkGreen, 14},  {(int)Qt::darkBlue, 15},
        {(int)Qt::darkCyan, 16},   {(int)Qt::darkMagenta, 17},
        {(int)Qt::darkYellow, 18}, {(int)Qt::transparent, 19},
    };

    for (int i = 0; i < 20; ++i) {
        s_global_color_lookup[global_color_vals[i].key] =
            global_color_vals[i].val;
    }

    static KeyVal hinting_preference_vals[] = {
        {(int)QFont::PreferDefaultHinting, 0},
        {(int)QFont::PreferNoHinting, 1},
        {(int)QFont::PreferVerticalHinting, 2},
        {(int)QFont::PreferFullHinting, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_hinting_preference_lookup[hinting_preference_vals[i].key] =
            hinting_preference_vals[i].val;
    }

    static KeyVal hit_test_accuracy_vals[] = {
        {(int)Qt::ExactHit, 0},
        {(int)Qt::FuzzyHit, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_hit_test_accuracy_lookup[hit_test_accuracy_vals[i].key] =
            hit_test_accuracy_vals[i].val;
    }

    static KeyVal image_conversion_flag_vals[] = {
        {(int)Qt::ColorMode_Mask, 0},
        {(int)Qt::AutoColor, 1},
        {(int)Qt::ColorOnly, 2},
        {(int)Qt::MonoOnly, 3},
        {(int)Qt::AlphaDither_Mask, 4},
        {(int)Qt::ThresholdAlphaDither, 5},
        {(int)Qt::OrderedAlphaDither, 6},
        {(int)Qt::DiffuseAlphaDither, 7},
        {(int)Qt::NoAlpha, 8},
        {(int)Qt::Dither_Mask, 9},
        {(int)Qt::DiffuseDither, 10},
        {(int)Qt::OrderedDither, 11},
        {(int)Qt::ThresholdDither, 12},
        {(int)Qt::DitherMode_Mask, 13},
        {(int)Qt::AutoDither, 14},
        {(int)Qt::PreferDither, 15},
        {(int)Qt::AvoidDither, 16},
        {(int)Qt::NoOpaqueDetection, 17},
        {(int)Qt::NoFormatConversion, 18},
    };

    for (int i = 0; i < 19; ++i) {
        s_image_conversion_flag_lookup[image_conversion_flag_vals[i].key] =
            image_conversion_flag_vals[i].val;
    }

    static KeyVal initialization_vals[] = {
        {(int)Qt::Uninitialized, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_initialization_lookup[initialization_vals[i].key] =
            initialization_vals[i].val;
    }

    static KeyVal input_method_hint_vals[] = {
        {(int)Qt::ImhNone, 0},
        {(int)Qt::ImhHiddenText, 1},
        {(int)Qt::ImhSensitiveData, 2},
        {(int)Qt::ImhNoAutoUppercase, 3},
        {(int)Qt::ImhPreferNumbers, 4},
        {(int)Qt::ImhPreferUppercase, 5},
        {(int)Qt::ImhPreferLowercase, 6},
        {(int)Qt::ImhNoPredictiveText, 7},
        {(int)Qt::ImhDate, 8},
        {(int)Qt::ImhTime, 9},
        {(int)Qt::ImhPreferLatin, 10},
        {(int)Qt::ImhMultiLine, 11},
        {(int)Qt::ImhNoEditMenu, 12},
        {(int)Qt::ImhNoTextHandles, 13},
        {(int)Qt::ImhDigitsOnly, 14},
        {(int)Qt::ImhFormattedNumbersOnly, 15},
        {(int)Qt::ImhUppercaseOnly, 16},
        {(int)Qt::ImhLowercaseOnly, 17},
        {(int)Qt::ImhDialableCharactersOnly, 18},
        {(int)Qt::ImhEmailCharactersOnly, 19},
        {(int)Qt::ImhUrlCharactersOnly, 20},
        {(int)Qt::ImhLatinOnly, 21},
        {(int)Qt::ImhExclusiveInputMask, 22},
    };

    for (int i = 0; i < 23; ++i) {
        s_input_method_hint_lookup[input_method_hint_vals[i].key] =
            input_method_hint_vals[i].val;
    }

    static KeyVal input_method_query_vals[] = {
        {(int)Qt::ImEnabled, 0},
        {(int)Qt::ImCursorRectangle, 1},
        {(int)Qt::ImMicroFocus, 2},
        {(int)Qt::ImFont, 3},
        {(int)Qt::ImCursorPosition, 4},
        {(int)Qt::ImSurroundingText, 5},
        {(int)Qt::ImCurrentSelection, 6},
        {(int)Qt::ImMaximumTextLength, 7},
        {(int)Qt::ImAnchorPosition, 8},
        {(int)Qt::ImHints, 9},
        {(int)Qt::ImPreferredLanguage, 10},
        {(int)Qt::ImAbsolutePosition, 11},
        {(int)Qt::ImTextBeforeCursor, 12},
        {(int)Qt::ImTextAfterCursor, 13},
        {(int)Qt::ImEnterKeyType, 14},
        {(int)Qt::ImAnchorRectangle, 15},
        {(int)Qt::ImInputItemClipRectangle, 16},
        {(int)Qt::ImPlatformData, 17},
        {(int)Qt::ImQueryInput, 18},
        {(int)Qt::ImQueryAll, 19},
    };

    for (int i = 0; i < 20; ++i) {
        s_input_method_query_lookup[input_method_query_vals[i].key] =
            input_method_query_vals[i].val;
    }

    static KeyVal interpolation_mode_vals[] = {
        {(int)QGradient::ColorInterpolation, 0},
        {(int)QGradient::ComponentInterpolation, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_interpolation_mode_lookup[interpolation_mode_vals[i].key] =
            interpolation_mode_vals[i].val;
    }

    static KeyVal intersect_type_vals[] = {
        {(int)QLineF::NoIntersection, 0},
        {(int)QLineF::BoundedIntersection, 1},
        {(int)QLineF::UnboundedIntersection, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_intersect_type_lookup[intersect_type_vals[i].key] =
            intersect_type_vals[i].val;
    }

    static KeyVal invert_mode_vals[] = {
        {(int)QImage::InvertRgb, 0},
        {(int)QImage::InvertRgba, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_invert_mode_lookup[invert_mode_vals[i].key] = invert_mode_vals[i].val;
    }

    static KeyVal item_data_role_vals[] = {
        {(int)Qt::DisplayRole, 0},
        {(int)Qt::DecorationRole, 1},
        {(int)Qt::EditRole, 2},
        {(int)Qt::ToolTipRole, 3},
        {(int)Qt::StatusTipRole, 4},
        {(int)Qt::WhatsThisRole, 5},
        {(int)Qt::FontRole, 6},
        {(int)Qt::TextAlignmentRole, 7},
        {(int)Qt::BackgroundColorRole, 8},
        {(int)Qt::BackgroundRole, 9},
        {(int)Qt::TextColorRole, 10},
        {(int)Qt::ForegroundRole, 11},
        {(int)Qt::CheckStateRole, 12},
        {(int)Qt::AccessibleTextRole, 13},
        {(int)Qt::AccessibleDescriptionRole, 14},
        {(int)Qt::SizeHintRole, 15},
        {(int)Qt::InitialSortOrderRole, 16},
        {(int)Qt::DisplayPropertyRole, 17},
        {(int)Qt::DecorationPropertyRole, 18},
        {(int)Qt::ToolTipPropertyRole, 19},
        {(int)Qt::StatusTipPropertyRole, 20},
        {(int)Qt::WhatsThisPropertyRole, 21},
        {(int)Qt::UserRole, 22},
    };

    for (int i = 0; i < 23; ++i) {
        s_item_data_role_lookup[item_data_role_vals[i].key] =
            item_data_role_vals[i].val;
    }

    static KeyVal item_flag_vals[] = {
        {(int)Qt::NoItemFlags, 0},         {(int)Qt::ItemIsSelectable, 1},
        {(int)Qt::ItemIsEditable, 2},      {(int)Qt::ItemIsDragEnabled, 3},
        {(int)Qt::ItemIsDropEnabled, 4},   {(int)Qt::ItemIsUserCheckable, 5},
        {(int)Qt::ItemIsEnabled, 6},       {(int)Qt::ItemIsAutoTristate, 7},
        {(int)Qt::ItemIsTristate, 8},      {(int)Qt::ItemNeverHasChildren, 9},
        {(int)Qt::ItemIsUserTristate, 10},
    };

    for (int i = 0; i < 11; ++i) {
        s_item_flag_lookup[item_flag_vals[i].key] = item_flag_vals[i].val;
    }

    static KeyVal item_selection_mode_vals[] = {
        {(int)Qt::ContainsItemShape, 0},
        {(int)Qt::IntersectsItemShape, 1},
        {(int)Qt::ContainsItemBoundingRect, 2},
        {(int)Qt::IntersectsItemBoundingRect, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_item_selection_mode_lookup[item_selection_mode_vals[i].key] =
            item_selection_mode_vals[i].val;
    }

    static KeyVal item_selection_operation_vals[] = {
        {(int)Qt::ReplaceSelection, 0},
        {(int)Qt::AddToSelection, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_item_selection_operation_lookup[item_selection_operation_vals[i]
                                              .key] =
            item_selection_operation_vals[i].val;
    }

    static KeyVal item_type_vals[] = {
        {(int)QListWidgetItem::Type, 0},
        {(int)QListWidgetItem::UserType, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_item_type_lookup[item_type_vals[i].key] = item_type_vals[i].val;
    }

    static KeyVal key_vals[] = {
        {(int)Qt::Key_Escape, 0},
        {(int)Qt::Key_Tab, 1},
        {(int)Qt::Key_Backtab, 2},
        {(int)Qt::Key_Backspace, 3},
        {(int)Qt::Key_Return, 4},
        {(int)Qt::Key_Enter, 5},
        {(int)Qt::Key_Insert, 6},
        {(int)Qt::Key_Delete, 7},
        {(int)Qt::Key_Pause, 8},
        {(int)Qt::Key_Print, 9},
        {(int)Qt::Key_SysReq, 10},
        {(int)Qt::Key_Clear, 11},
        {(int)Qt::Key_Home, 12},
        {(int)Qt::Key_End, 13},
        {(int)Qt::Key_Left, 14},
        {(int)Qt::Key_Up, 15},
        {(int)Qt::Key_Right, 16},
        {(int)Qt::Key_Down, 17},
        {(int)Qt::Key_PageUp, 18},
        {(int)Qt::Key_PageDown, 19},
        {(int)Qt::Key_Shift, 20},
        {(int)Qt::Key_Control, 21},
        {(int)Qt::Key_Meta, 22},
        {(int)Qt::Key_Alt, 23},
        {(int)Qt::Key_CapsLock, 24},
        {(int)Qt::Key_NumLock, 25},
        {(int)Qt::Key_ScrollLock, 26},
        {(int)Qt::Key_F1, 27},
        {(int)Qt::Key_F2, 28},
        {(int)Qt::Key_F3, 29},
        {(int)Qt::Key_F4, 30},
        {(int)Qt::Key_F5, 31},
        {(int)Qt::Key_F6, 32},
        {(int)Qt::Key_F7, 33},
        {(int)Qt::Key_F8, 34},
        {(int)Qt::Key_F9, 35},
        {(int)Qt::Key_F10, 36},
        {(int)Qt::Key_F11, 37},
        {(int)Qt::Key_F12, 38},
        {(int)Qt::Key_F13, 39},
        {(int)Qt::Key_F14, 40},
        {(int)Qt::Key_F15, 41},
        {(int)Qt::Key_F16, 42},
        {(int)Qt::Key_F17, 43},
        {(int)Qt::Key_F18, 44},
        {(int)Qt::Key_F19, 45},
        {(int)Qt::Key_F20, 46},
        {(int)Qt::Key_F21, 47},
        {(int)Qt::Key_F22, 48},
        {(int)Qt::Key_F23, 49},
        {(int)Qt::Key_F24, 50},
        {(int)Qt::Key_F25, 51},
        {(int)Qt::Key_F26, 52},
        {(int)Qt::Key_F27, 53},
        {(int)Qt::Key_F28, 54},
        {(int)Qt::Key_F29, 55},
        {(int)Qt::Key_F30, 56},
        {(int)Qt::Key_F31, 57},
        {(int)Qt::Key_F32, 58},
        {(int)Qt::Key_F33, 59},
        {(int)Qt::Key_F34, 60},
        {(int)Qt::Key_F35, 61},
        {(int)Qt::Key_Super_L, 62},
        {(int)Qt::Key_Super_R, 63},
        {(int)Qt::Key_Menu, 64},
        {(int)Qt::Key_Hyper_L, 65},
        {(int)Qt::Key_Hyper_R, 66},
        {(int)Qt::Key_Help, 67},
        {(int)Qt::Key_Direction_L, 68},
        {(int)Qt::Key_Direction_R, 69},
        {(int)Qt::Key_Space, 70},
        {(int)Qt::Key_Any, 71},
        {(int)Qt::Key_Exclam, 72},
        {(int)Qt::Key_QuoteDbl, 73},
        {(int)Qt::Key_NumberSign, 74},
        {(int)Qt::Key_Dollar, 75},
        {(int)Qt::Key_Percent, 76},
        {(int)Qt::Key_Ampersand, 77},
        {(int)Qt::Key_Apostrophe, 78},
        {(int)Qt::Key_ParenLeft, 79},
        {(int)Qt::Key_ParenRight, 80},
        {(int)Qt::Key_Asterisk, 81},
        {(int)Qt::Key_Plus, 82},
        {(int)Qt::Key_Comma, 83},
        {(int)Qt::Key_Minus, 84},
        {(int)Qt::Key_Period, 85},
        {(int)Qt::Key_Slash, 86},
        {(int)Qt::Key_0, 87},
        {(int)Qt::Key_1, 88},
        {(int)Qt::Key_2, 89},
        {(int)Qt::Key_3, 90},
        {(int)Qt::Key_4, 91},
        {(int)Qt::Key_5, 92},
        {(int)Qt::Key_6, 93},
        {(int)Qt::Key_7, 94},
        {(int)Qt::Key_8, 95},
        {(int)Qt::Key_9, 96},
        {(int)Qt::Key_Colon, 97},
        {(int)Qt::Key_Semicolon, 98},
        {(int)Qt::Key_Less, 99},
        {(int)Qt::Key_Equal, 100},
        {(int)Qt::Key_Greater, 101},
        {(int)Qt::Key_Question, 102},
        {(int)Qt::Key_At, 103},
        {(int)Qt::Key_A, 104},
        {(int)Qt::Key_B, 105},
        {(int)Qt::Key_C, 106},
        {(int)Qt::Key_D, 107},
        {(int)Qt::Key_E, 108},
        {(int)Qt::Key_F, 109},
        {(int)Qt::Key_G, 110},
        {(int)Qt::Key_H, 111},
        {(int)Qt::Key_I, 112},
        {(int)Qt::Key_J, 113},
        {(int)Qt::Key_K, 114},
        {(int)Qt::Key_L, 115},
        {(int)Qt::Key_M, 116},
        {(int)Qt::Key_N, 117},
        {(int)Qt::Key_O, 118},
        {(int)Qt::Key_P, 119},
        {(int)Qt::Key_Q, 120},
        {(int)Qt::Key_R, 121},
        {(int)Qt::Key_S, 122},
        {(int)Qt::Key_T, 123},
        {(int)Qt::Key_U, 124},
        {(int)Qt::Key_V, 125},
        {(int)Qt::Key_W, 126},
        {(int)Qt::Key_X, 127},
        {(int)Qt::Key_Y, 128},
        {(int)Qt::Key_Z, 129},
        {(int)Qt::Key_BracketLeft, 130},
        {(int)Qt::Key_Backslash, 131},
        {(int)Qt::Key_BracketRight, 132},
        {(int)Qt::Key_AsciiCircum, 133},
        {(int)Qt::Key_Underscore, 134},
        {(int)Qt::Key_QuoteLeft, 135},
        {(int)Qt::Key_BraceLeft, 136},
        {(int)Qt::Key_Bar, 137},
        {(int)Qt::Key_BraceRight, 138},
        {(int)Qt::Key_AsciiTilde, 139},
        {(int)Qt::Key_nobreakspace, 140},
        {(int)Qt::Key_exclamdown, 141},
        {(int)Qt::Key_cent, 142},
        {(int)Qt::Key_sterling, 143},
        {(int)Qt::Key_currency, 144},
        {(int)Qt::Key_yen, 145},
        {(int)Qt::Key_brokenbar, 146},
        {(int)Qt::Key_section, 147},
        {(int)Qt::Key_diaeresis, 148},
        {(int)Qt::Key_copyright, 149},
        {(int)Qt::Key_ordfeminine, 150},
        {(int)Qt::Key_guillemotleft, 151},
        {(int)Qt::Key_notsign, 152},
        {(int)Qt::Key_hyphen, 153},
        {(int)Qt::Key_registered, 154},
        {(int)Qt::Key_macron, 155},
        {(int)Qt::Key_degree, 156},
        {(int)Qt::Key_plusminus, 157},
        {(int)Qt::Key_twosuperior, 158},
        {(int)Qt::Key_threesuperior, 159},
        {(int)Qt::Key_acute, 160},
        {(int)Qt::Key_mu, 161},
        {(int)Qt::Key_paragraph, 162},
        {(int)Qt::Key_periodcentered, 163},
        {(int)Qt::Key_cedilla, 164},
        {(int)Qt::Key_onesuperior, 165},
        {(int)Qt::Key_masculine, 166},
        {(int)Qt::Key_guillemotright, 167},
        {(int)Qt::Key_onequarter, 168},
        {(int)Qt::Key_onehalf, 169},
        {(int)Qt::Key_threequarters, 170},
        {(int)Qt::Key_questiondown, 171},
        {(int)Qt::Key_Agrave, 172},
        {(int)Qt::Key_Aacute, 173},
        {(int)Qt::Key_Acircumflex, 174},
        {(int)Qt::Key_Atilde, 175},
        {(int)Qt::Key_Adiaeresis, 176},
        {(int)Qt::Key_Aring, 177},
        {(int)Qt::Key_AE, 178},
        {(int)Qt::Key_Ccedilla, 179},
        {(int)Qt::Key_Egrave, 180},
        {(int)Qt::Key_Eacute, 181},
        {(int)Qt::Key_Ecircumflex, 182},
        {(int)Qt::Key_Ediaeresis, 183},
        {(int)Qt::Key_Igrave, 184},
        {(int)Qt::Key_Iacute, 185},
        {(int)Qt::Key_Icircumflex, 186},
        {(int)Qt::Key_Idiaeresis, 187},
        {(int)Qt::Key_ETH, 188},
        {(int)Qt::Key_Ntilde, 189},
        {(int)Qt::Key_Ograve, 190},
        {(int)Qt::Key_Oacute, 191},
        {(int)Qt::Key_Ocircumflex, 192},
        {(int)Qt::Key_Otilde, 193},
        {(int)Qt::Key_Odiaeresis, 194},
        {(int)Qt::Key_multiply, 195},
        {(int)Qt::Key_Ooblique, 196},
        {(int)Qt::Key_Ugrave, 197},
        {(int)Qt::Key_Uacute, 198},
        {(int)Qt::Key_Ucircumflex, 199},
        {(int)Qt::Key_Udiaeresis, 200},
        {(int)Qt::Key_Yacute, 201},
        {(int)Qt::Key_THORN, 202},
        {(int)Qt::Key_ssharp, 203},
        {(int)Qt::Key_division, 204},
        {(int)Qt::Key_ydiaeresis, 205},
        {(int)Qt::Key_AltGr, 206},
        {(int)Qt::Key_Multi_key, 207},
        {(int)Qt::Key_Codeinput, 208},
        {(int)Qt::Key_SingleCandidate, 209},
        {(int)Qt::Key_MultipleCandidate, 210},
        {(int)Qt::Key_PreviousCandidate, 211},
        {(int)Qt::Key_Mode_switch, 212},
        {(int)Qt::Key_Kanji, 213},
        {(int)Qt::Key_Muhenkan, 214},
        {(int)Qt::Key_Henkan, 215},
        {(int)Qt::Key_Romaji, 216},
        {(int)Qt::Key_Hiragana, 217},
        {(int)Qt::Key_Katakana, 218},
        {(int)Qt::Key_Hiragana_Katakana, 219},
        {(int)Qt::Key_Zenkaku, 220},
        {(int)Qt::Key_Hankaku, 221},
        {(int)Qt::Key_Zenkaku_Hankaku, 222},
        {(int)Qt::Key_Touroku, 223},
        {(int)Qt::Key_Massyo, 224},
        {(int)Qt::Key_Kana_Lock, 225},
        {(int)Qt::Key_Kana_Shift, 226},
        {(int)Qt::Key_Eisu_Shift, 227},
        {(int)Qt::Key_Eisu_toggle, 228},
        {(int)Qt::Key_Hangul, 229},
        {(int)Qt::Key_Hangul_Start, 230},
        {(int)Qt::Key_Hangul_End, 231},
        {(int)Qt::Key_Hangul_Hanja, 232},
        {(int)Qt::Key_Hangul_Jamo, 233},
        {(int)Qt::Key_Hangul_Romaja, 234},
        {(int)Qt::Key_Hangul_Jeonja, 235},
        {(int)Qt::Key_Hangul_Banja, 236},
        {(int)Qt::Key_Hangul_PreHanja, 237},
        {(int)Qt::Key_Hangul_PostHanja, 238},
        {(int)Qt::Key_Hangul_Special, 239},
        {(int)Qt::Key_Dead_Grave, 240},
        {(int)Qt::Key_Dead_Acute, 241},
        {(int)Qt::Key_Dead_Circumflex, 242},
        {(int)Qt::Key_Dead_Tilde, 243},
        {(int)Qt::Key_Dead_Macron, 244},
        {(int)Qt::Key_Dead_Breve, 245},
        {(int)Qt::Key_Dead_Abovedot, 246},
        {(int)Qt::Key_Dead_Diaeresis, 247},
        {(int)Qt::Key_Dead_Abovering, 248},
        {(int)Qt::Key_Dead_Doubleacute, 249},
        {(int)Qt::Key_Dead_Caron, 250},
        {(int)Qt::Key_Dead_Cedilla, 251},
        {(int)Qt::Key_Dead_Ogonek, 252},
        {(int)Qt::Key_Dead_Iota, 253},
        {(int)Qt::Key_Dead_Voiced_Sound, 254},
        {(int)Qt::Key_Dead_Semivoiced_Sound, 255},
        {(int)Qt::Key_Dead_Belowdot, 256},
        {(int)Qt::Key_Dead_Hook, 257},
        {(int)Qt::Key_Dead_Horn, 258},
        {(int)Qt::Key_Dead_Stroke, 259},
        {(int)Qt::Key_Dead_Abovecomma, 260},
        {(int)Qt::Key_Dead_Abovereversedcomma, 261},
        {(int)Qt::Key_Dead_Doublegrave, 262},
        {(int)Qt::Key_Dead_Belowring, 263},
        {(int)Qt::Key_Dead_Belowmacron, 264},
        {(int)Qt::Key_Dead_Belowcircumflex, 265},
        {(int)Qt::Key_Dead_Belowtilde, 266},
        {(int)Qt::Key_Dead_Belowbreve, 267},
        {(int)Qt::Key_Dead_Belowdiaeresis, 268},
        {(int)Qt::Key_Dead_Invertedbreve, 269},
        {(int)Qt::Key_Dead_Belowcomma, 270},
        {(int)Qt::Key_Dead_Currency, 271},
        {(int)Qt::Key_Dead_A, 272},
        {(int)Qt::Key_Dead_E, 273},
        {(int)Qt::Key_Dead_I, 274},
        {(int)Qt::Key_Dead_O, 275},
        {(int)Qt::Key_Dead_U, 276},
        {(int)Qt::Key_Dead_Small_Schwa, 277},
        {(int)Qt::Key_Dead_Capital_Schwa, 278},
        {(int)Qt::Key_Dead_Greek, 279},
        {(int)Qt::Key_Dead_Lowline, 280},
        {(int)Qt::Key_Dead_Aboveverticalline, 281},
        {(int)Qt::Key_Dead_Belowverticalline, 282},
        {(int)Qt::Key_Dead_Longsolidusoverlay, 283},
        {(int)Qt::Key_Back, 284},
        {(int)Qt::Key_Forward, 285},
        {(int)Qt::Key_Stop, 286},
        {(int)Qt::Key_Refresh, 287},
        {(int)Qt::Key_VolumeDown, 288},
        {(int)Qt::Key_VolumeMute, 289},
        {(int)Qt::Key_VolumeUp, 290},
        {(int)Qt::Key_BassBoost, 291},
        {(int)Qt::Key_BassUp, 292},
        {(int)Qt::Key_BassDown, 293},
        {(int)Qt::Key_TrebleUp, 294},
        {(int)Qt::Key_TrebleDown, 295},
        {(int)Qt::Key_MediaPlay, 296},
        {(int)Qt::Key_MediaStop, 297},
        {(int)Qt::Key_MediaPrevious, 298},
        {(int)Qt::Key_MediaNext, 299},
        {(int)Qt::Key_MediaRecord, 300},
        {(int)Qt::Key_MediaPause, 301},
        {(int)Qt::Key_MediaTogglePlayPause, 302},
        {(int)Qt::Key_HomePage, 303},
        {(int)Qt::Key_Favorites, 304},
        {(int)Qt::Key_Search, 305},
        {(int)Qt::Key_Standby, 306},
        {(int)Qt::Key_OpenUrl, 307},
        {(int)Qt::Key_LaunchMail, 308},
        {(int)Qt::Key_LaunchMedia, 309},
        {(int)Qt::Key_Launch0, 310},
        {(int)Qt::Key_Launch1, 311},
        {(int)Qt::Key_Launch2, 312},
        {(int)Qt::Key_Launch3, 313},
        {(int)Qt::Key_Launch4, 314},
        {(int)Qt::Key_Launch5, 315},
        {(int)Qt::Key_Launch6, 316},
        {(int)Qt::Key_Launch7, 317},
        {(int)Qt::Key_Launch8, 318},
        {(int)Qt::Key_Launch9, 319},
        {(int)Qt::Key_LaunchA, 320},
        {(int)Qt::Key_LaunchB, 321},
        {(int)Qt::Key_LaunchC, 322},
        {(int)Qt::Key_LaunchD, 323},
        {(int)Qt::Key_LaunchE, 324},
        {(int)Qt::Key_LaunchF, 325},
        {(int)Qt::Key_MonBrightnessUp, 326},
        {(int)Qt::Key_MonBrightnessDown, 327},
        {(int)Qt::Key_KeyboardLightOnOff, 328},
        {(int)Qt::Key_KeyboardBrightnessUp, 329},
        {(int)Qt::Key_KeyboardBrightnessDown, 330},
        {(int)Qt::Key_PowerOff, 331},
        {(int)Qt::Key_WakeUp, 332},
        {(int)Qt::Key_Eject, 333},
        {(int)Qt::Key_ScreenSaver, 334},
        {(int)Qt::Key_WWW, 335},
        {(int)Qt::Key_Memo, 336},
        {(int)Qt::Key_LightBulb, 337},
        {(int)Qt::Key_Shop, 338},
        {(int)Qt::Key_History, 339},
        {(int)Qt::Key_AddFavorite, 340},
        {(int)Qt::Key_HotLinks, 341},
        {(int)Qt::Key_BrightnessAdjust, 342},
        {(int)Qt::Key_Finance, 343},
        {(int)Qt::Key_Community, 344},
        {(int)Qt::Key_AudioRewind, 345},
        {(int)Qt::Key_BackForward, 346},
        {(int)Qt::Key_ApplicationLeft, 347},
        {(int)Qt::Key_ApplicationRight, 348},
        {(int)Qt::Key_Book, 349},
        {(int)Qt::Key_CD, 350},
        {(int)Qt::Key_Calculator, 351},
        {(int)Qt::Key_ToDoList, 352},
        {(int)Qt::Key_ClearGrab, 353},
        {(int)Qt::Key_Close, 354},
        {(int)Qt::Key_Copy, 355},
        {(int)Qt::Key_Cut, 356},
        {(int)Qt::Key_Display, 357},
        {(int)Qt::Key_DOS, 358},
        {(int)Qt::Key_Documents, 359},
        {(int)Qt::Key_Excel, 360},
        {(int)Qt::Key_Explorer, 361},
        {(int)Qt::Key_Game, 362},
        {(int)Qt::Key_Go, 363},
        {(int)Qt::Key_iTouch, 364},
        {(int)Qt::Key_LogOff, 365},
        {(int)Qt::Key_Market, 366},
        {(int)Qt::Key_Meeting, 367},
        {(int)Qt::Key_MenuKB, 368},
        {(int)Qt::Key_MenuPB, 369},
        {(int)Qt::Key_MySites, 370},
        {(int)Qt::Key_News, 371},
        {(int)Qt::Key_OfficeHome, 372},
        {(int)Qt::Key_Option, 373},
        {(int)Qt::Key_Paste, 374},
        {(int)Qt::Key_Phone, 375},
        {(int)Qt::Key_Calendar, 376},
        {(int)Qt::Key_Reply, 377},
        {(int)Qt::Key_Reload, 378},
        {(int)Qt::Key_RotateWindows, 379},
        {(int)Qt::Key_RotationPB, 380},
        {(int)Qt::Key_RotationKB, 381},
        {(int)Qt::Key_Save, 382},
        {(int)Qt::Key_Send, 383},
        {(int)Qt::Key_Spell, 384},
        {(int)Qt::Key_SplitScreen, 385},
        {(int)Qt::Key_Support, 386},
        {(int)Qt::Key_TaskPane, 387},
        {(int)Qt::Key_Terminal, 388},
        {(int)Qt::Key_Tools, 389},
        {(int)Qt::Key_Travel, 390},
        {(int)Qt::Key_Video, 391},
        {(int)Qt::Key_Word, 392},
        {(int)Qt::Key_Xfer, 393},
        {(int)Qt::Key_ZoomIn, 394},
        {(int)Qt::Key_ZoomOut, 395},
        {(int)Qt::Key_Away, 396},
        {(int)Qt::Key_Messenger, 397},
        {(int)Qt::Key_WebCam, 398},
        {(int)Qt::Key_MailForward, 399},
        {(int)Qt::Key_Pictures, 400},
        {(int)Qt::Key_Music, 401},
        {(int)Qt::Key_Battery, 402},
        {(int)Qt::Key_Bluetooth, 403},
        {(int)Qt::Key_WLAN, 404},
        {(int)Qt::Key_UWB, 405},
        {(int)Qt::Key_AudioForward, 406},
        {(int)Qt::Key_AudioRepeat, 407},
        {(int)Qt::Key_AudioRandomPlay, 408},
        {(int)Qt::Key_Subtitle, 409},
        {(int)Qt::Key_AudioCycleTrack, 410},
        {(int)Qt::Key_Time, 411},
        {(int)Qt::Key_Hibernate, 412},
        {(int)Qt::Key_View, 413},
        {(int)Qt::Key_TopMenu, 414},
        {(int)Qt::Key_PowerDown, 415},
        {(int)Qt::Key_Suspend, 416},
        {(int)Qt::Key_ContrastAdjust, 417},
        {(int)Qt::Key_LaunchG, 418},
        {(int)Qt::Key_LaunchH, 419},
        {(int)Qt::Key_TouchpadToggle, 420},
        {(int)Qt::Key_TouchpadOn, 421},
        {(int)Qt::Key_TouchpadOff, 422},
        {(int)Qt::Key_MicMute, 423},
        {(int)Qt::Key_Red, 424},
        {(int)Qt::Key_Green, 425},
        {(int)Qt::Key_Yellow, 426},
        {(int)Qt::Key_Blue, 427},
        {(int)Qt::Key_ChannelUp, 428},
        {(int)Qt::Key_ChannelDown, 429},
        {(int)Qt::Key_Guide, 430},
        {(int)Qt::Key_Info, 431},
        {(int)Qt::Key_Settings, 432},
        {(int)Qt::Key_MicVolumeUp, 433},
        {(int)Qt::Key_MicVolumeDown, 434},
        {(int)Qt::Key_New, 435},
        {(int)Qt::Key_Open, 436},
        {(int)Qt::Key_Find, 437},
        {(int)Qt::Key_Undo, 438},
        {(int)Qt::Key_Redo, 439},
        {(int)Qt::Key_MediaLast, 440},
        {(int)Qt::Key_Select, 441},
        {(int)Qt::Key_Yes, 442},
        {(int)Qt::Key_No, 443},
        {(int)Qt::Key_Cancel, 444},
        {(int)Qt::Key_Printer, 445},
        {(int)Qt::Key_Execute, 446},
        {(int)Qt::Key_Sleep, 447},
        {(int)Qt::Key_Play, 448},
        {(int)Qt::Key_Zoom, 449},
        {(int)Qt::Key_Exit, 450},
        {(int)Qt::Key_Context1, 451},
        {(int)Qt::Key_Context2, 452},
        {(int)Qt::Key_Context3, 453},
        {(int)Qt::Key_Context4, 454},
        {(int)Qt::Key_Call, 455},
        {(int)Qt::Key_Hangup, 456},
        {(int)Qt::Key_Flip, 457},
        {(int)Qt::Key_ToggleCallHangup, 458},
        {(int)Qt::Key_VoiceDial, 459},
        {(int)Qt::Key_LastNumberRedial, 460},
        {(int)Qt::Key_Camera, 461},
        {(int)Qt::Key_CameraFocus, 462},
        {(int)Qt::Key_unknown, 463},
    };

    for (int i = 0; i < 464; ++i) {
        s_key_lookup[key_vals[i].key] = key_vals[i].val;
    }

    static KeyVal keyboard_modifier_vals[] = {
        {(int)Qt::NoModifier, 0},          {(int)Qt::ShiftModifier, 1},
        {(int)Qt::ControlModifier, 2},     {(int)Qt::AltModifier, 3},
        {(int)Qt::MetaModifier, 4},        {(int)Qt::KeypadModifier, 5},
        {(int)Qt::GroupSwitchModifier, 6}, {(int)Qt::KeyboardModifierMask, 7},
    };

    for (int i = 0; i < 8; ++i) {
        s_keyboard_modifier_lookup[keyboard_modifier_vals[i].key] =
            keyboard_modifier_vals[i].val;
    }

    static KeyVal layout_direction_vals[] = {
        {(int)Qt::LeftToRight, 0},
        {(int)Qt::RightToLeft, 1},
        {(int)Qt::LayoutDirectionAuto, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_layout_direction_lookup[layout_direction_vals[i].key] =
            layout_direction_vals[i].val;
    }

    static KeyVal mask_mode_vals[] = {
        {(int)Qt::MaskInColor, 0},
        {(int)Qt::MaskOutColor, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_mask_mode_lookup[mask_mode_vals[i].key] = mask_mode_vals[i].val;
    }

    static KeyVal match_flag_vals[] = {
        {(int)Qt::MatchExactly, 0},     {(int)Qt::MatchContains, 1},
        {(int)Qt::MatchStartsWith, 2},  {(int)Qt::MatchEndsWith, 3},
        {(int)Qt::MatchRegExp, 4},      {(int)Qt::MatchWildcard, 5},
        {(int)Qt::MatchFixedString, 6}, {(int)Qt::MatchCaseSensitive, 7},
        {(int)Qt::MatchWrap, 8},        {(int)Qt::MatchRecursive, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_match_flag_lookup[match_flag_vals[i].key] = match_flag_vals[i].val;
    }

    static KeyVal mode_vals[] = {
        {(int)QIcon::Normal, 0},
        {(int)QIcon::Disabled, 1},
        {(int)QIcon::Active, 2},
        {(int)QIcon::Selected, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_mode_lookup[mode_vals[i].key] = mode_vals[i].val;
    }

    static KeyVal modifier_vals[] = {
        {(int)Qt::META, 0},          {(int)Qt::SHIFT, 1},
        {(int)Qt::CTRL, 2},          {(int)Qt::ALT, 3},
        {(int)Qt::MODIFIER_MASK, 4}, {(int)Qt::UNICODE_ACCEL, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_modifier_lookup[modifier_vals[i].key] = modifier_vals[i].val;
    }

    static KeyVal mouse_button_vals[] = {
        {(int)Qt::NoButton, 0},         {(int)Qt::LeftButton, 1},
        {(int)Qt::RightButton, 2},      {(int)Qt::MidButton, 3},
        {(int)Qt::MiddleButton, 4},     {(int)Qt::BackButton, 5},
        {(int)Qt::XButton1, 6},         {(int)Qt::ExtraButton1, 7},
        {(int)Qt::ForwardButton, 8},    {(int)Qt::XButton2, 9},
        {(int)Qt::ExtraButton2, 10},    {(int)Qt::TaskButton, 11},
        {(int)Qt::ExtraButton3, 12},    {(int)Qt::ExtraButton4, 13},
        {(int)Qt::ExtraButton5, 14},    {(int)Qt::ExtraButton6, 15},
        {(int)Qt::ExtraButton7, 16},    {(int)Qt::ExtraButton8, 17},
        {(int)Qt::ExtraButton9, 18},    {(int)Qt::ExtraButton10, 19},
        {(int)Qt::ExtraButton11, 20},   {(int)Qt::ExtraButton12, 21},
        {(int)Qt::ExtraButton13, 22},   {(int)Qt::ExtraButton14, 23},
        {(int)Qt::ExtraButton15, 24},   {(int)Qt::ExtraButton16, 25},
        {(int)Qt::ExtraButton17, 26},   {(int)Qt::ExtraButton18, 27},
        {(int)Qt::ExtraButton19, 28},   {(int)Qt::ExtraButton20, 29},
        {(int)Qt::ExtraButton21, 30},   {(int)Qt::ExtraButton22, 31},
        {(int)Qt::ExtraButton23, 32},   {(int)Qt::ExtraButton24, 33},
        {(int)Qt::AllButtons, 34},      {(int)Qt::MaxMouseButton, 35},
        {(int)Qt::MouseButtonMask, 36},
    };

    for (int i = 0; i < 37; ++i) {
        s_mouse_button_lookup[mouse_button_vals[i].key] =
            mouse_button_vals[i].val;
    }

    static KeyVal mouse_event_flag_vals[] = {
        {(int)Qt::MouseEventCreatedDoubleClick, 0},
        {(int)Qt::MouseEventFlagMask, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_mouse_event_flag_lookup[mouse_event_flag_vals[i].key] =
            mouse_event_flag_vals[i].val;
    }

    static KeyVal mouse_event_source_vals[] = {
        {(int)Qt::MouseEventNotSynthesized, 0},
        {(int)Qt::MouseEventSynthesizedBySystem, 1},
        {(int)Qt::MouseEventSynthesizedByQt, 2},
        {(int)Qt::MouseEventSynthesizedByApplication, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_mouse_event_source_lookup[mouse_event_source_vals[i].key] =
            mouse_event_source_vals[i].val;
    }

    static KeyVal name_format_vals[] = {
        {(int)QColor::HexRgb, 0},
        {(int)QColor::HexArgb, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_name_format_lookup[name_format_vals[i].key] = name_format_vals[i].val;
    }

    static KeyVal native_gesture_type_vals[] = {
        {(int)Qt::BeginNativeGesture, 0},     {(int)Qt::EndNativeGesture, 1},
        {(int)Qt::PanNativeGesture, 2},       {(int)Qt::ZoomNativeGesture, 3},
        {(int)Qt::SmartZoomNativeGesture, 4}, {(int)Qt::RotateNativeGesture, 5},
        {(int)Qt::SwipeNativeGesture, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_native_gesture_type_lookup[native_gesture_type_vals[i].key] =
            native_gesture_type_vals[i].val;
    }

    static KeyVal navigation_mode_vals[] = {
        {(int)Qt::NavigationModeNone, 0},
        {(int)Qt::NavigationModeKeypadTabOrder, 1},
        {(int)Qt::NavigationModeKeypadDirectional, 2},
        {(int)Qt::NavigationModeCursorAuto, 3},
        {(int)Qt::NavigationModeCursorForceVisible, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_navigation_mode_lookup[navigation_mode_vals[i].key] =
            navigation_mode_vals[i].val;
    }

    static KeyVal open_gl_context_profile_vals[] = {
        {(int)QSurfaceFormat::NoProfile, 0},
        {(int)QSurfaceFormat::CoreProfile, 1},
        {(int)QSurfaceFormat::CompatibilityProfile, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_open_gl_context_profile_lookup[open_gl_context_profile_vals[i].key] =
            open_gl_context_profile_vals[i].val;
    }

    static KeyVal orientation_vals[] = {
        {(int)Qt::Horizontal, 0},
        {(int)Qt::Vertical, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_orientation_lookup[orientation_vals[i].key] = orientation_vals[i].val;
    }

    static KeyVal paint_device_metric_vals[] = {
        {(int)QPaintDevice::PdmWidth, 0},
        {(int)QPaintDevice::PdmHeight, 1},
        {(int)QPaintDevice::PdmWidthMM, 2},
        {(int)QPaintDevice::PdmHeightMM, 3},
        {(int)QPaintDevice::PdmNumColors, 4},
        {(int)QPaintDevice::PdmDepth, 5},
        {(int)QPaintDevice::PdmDpiX, 6},
        {(int)QPaintDevice::PdmDpiY, 7},
        {(int)QPaintDevice::PdmPhysicalDpiX, 8},
        {(int)QPaintDevice::PdmPhysicalDpiY, 9},
        {(int)QPaintDevice::PdmDevicePixelRatio, 10},
        {(int)QPaintDevice::PdmDevicePixelRatioScaled, 11},
    };

    for (int i = 0; i < 12; ++i) {
        s_paint_device_metric_lookup[paint_device_metric_vals[i].key] =
            paint_device_metric_vals[i].val;
    }

    static KeyVal paint_engine_feature_vals[] = {
        {(int)QPaintEngine::PrimitiveTransform, 0},
        {(int)QPaintEngine::PatternTransform, 1},
        {(int)QPaintEngine::PixmapTransform, 2},
        {(int)QPaintEngine::PatternBrush, 3},
        {(int)QPaintEngine::LinearGradientFill, 4},
        {(int)QPaintEngine::RadialGradientFill, 5},
        {(int)QPaintEngine::ConicalGradientFill, 6},
        {(int)QPaintEngine::AlphaBlend, 7},
        {(int)QPaintEngine::PorterDuff, 8},
        {(int)QPaintEngine::PainterPaths, 9},
        {(int)QPaintEngine::Antialiasing, 10},
        {(int)QPaintEngine::BrushStroke, 11},
        {(int)QPaintEngine::ConstantOpacity, 12},
        {(int)QPaintEngine::MaskedBrush, 13},
        {(int)QPaintEngine::PerspectiveTransform, 14},
        {(int)QPaintEngine::BlendModes, 15},
        {(int)QPaintEngine::ObjectBoundingModeGradients, 16},
        {(int)QPaintEngine::RasterOpModes, 17},
        {(int)QPaintEngine::PaintOutsidePaintEvent, 18},
        {(int)QPaintEngine::AllFeatures, 19},
    };

    for (int i = 0; i < 20; ++i) {
        s_paint_engine_feature_lookup[paint_engine_feature_vals[i].key] =
            paint_engine_feature_vals[i].val;
    }

    static KeyVal pen_cap_style_vals[] = {
        {(int)Qt::FlatCap, 0},
        {(int)Qt::SquareCap, 1},
        {(int)Qt::RoundCap, 2},
        {(int)Qt::MPenCapStyle, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_pen_cap_style_lookup[pen_cap_style_vals[i].key] =
            pen_cap_style_vals[i].val;
    }

    static KeyVal pen_join_style_vals[] = {
        {(int)Qt::MiterJoin, 0},     {(int)Qt::BevelJoin, 1},
        {(int)Qt::RoundJoin, 2},     {(int)Qt::SvgMiterJoin, 3},
        {(int)Qt::MPenJoinStyle, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_pen_join_style_lookup[pen_join_style_vals[i].key] =
            pen_join_style_vals[i].val;
    }

    static KeyVal pen_style_vals[] = {
        {(int)Qt::NoPen, 0},          {(int)Qt::SolidLine, 1},
        {(int)Qt::DashLine, 2},       {(int)Qt::DotLine, 3},
        {(int)Qt::DashDotLine, 4},    {(int)Qt::DashDotDotLine, 5},
        {(int)Qt::CustomDashLine, 6}, {(int)Qt::MPenStyle, 7},
    };

    for (int i = 0; i < 8; ++i) {
        s_pen_style_lookup[pen_style_vals[i].key] = pen_style_vals[i].val;
    }

    static KeyVal pixel_metric_vals[] = {
        {(int)QStyle::PM_ButtonMargin, 0},
        {(int)QStyle::PM_ButtonDefaultIndicator, 1},
        {(int)QStyle::PM_MenuButtonIndicator, 2},
        {(int)QStyle::PM_ButtonShiftHorizontal, 3},
        {(int)QStyle::PM_ButtonShiftVertical, 4},
        {(int)QStyle::PM_DefaultFrameWidth, 5},
        {(int)QStyle::PM_SpinBoxFrameWidth, 6},
        {(int)QStyle::PM_ComboBoxFrameWidth, 7},
        {(int)QStyle::PM_MaximumDragDistance, 8},
        {(int)QStyle::PM_ScrollBarExtent, 9},
        {(int)QStyle::PM_ScrollBarSliderMin, 10},
        {(int)QStyle::PM_SliderThickness, 11},
        {(int)QStyle::PM_SliderControlThickness, 12},
        {(int)QStyle::PM_SliderLength, 13},
        {(int)QStyle::PM_SliderTickmarkOffset, 14},
        {(int)QStyle::PM_SliderSpaceAvailable, 15},
        {(int)QStyle::PM_DockWidgetSeparatorExtent, 16},
        {(int)QStyle::PM_DockWidgetHandleExtent, 17},
        {(int)QStyle::PM_DockWidgetFrameWidth, 18},
        {(int)QStyle::PM_TabBarTabOverlap, 19},
        {(int)QStyle::PM_TabBarTabHSpace, 20},
        {(int)QStyle::PM_TabBarTabVSpace, 21},
        {(int)QStyle::PM_TabBarBaseHeight, 22},
        {(int)QStyle::PM_TabBarBaseOverlap, 23},
        {(int)QStyle::PM_ProgressBarChunkWidth, 24},
        {(int)QStyle::PM_SplitterWidth, 25},
        {(int)QStyle::PM_TitleBarHeight, 26},
        {(int)QStyle::PM_MenuScrollerHeight, 27},
        {(int)QStyle::PM_MenuHMargin, 28},
        {(int)QStyle::PM_MenuVMargin, 29},
        {(int)QStyle::PM_MenuPanelWidth, 30},
        {(int)QStyle::PM_MenuTearoffHeight, 31},
        {(int)QStyle::PM_MenuDesktopFrameWidth, 32},
        {(int)QStyle::PM_MenuBarPanelWidth, 33},
        {(int)QStyle::PM_MenuBarItemSpacing, 34},
        {(int)QStyle::PM_MenuBarVMargin, 35},
        {(int)QStyle::PM_MenuBarHMargin, 36},
        {(int)QStyle::PM_IndicatorWidth, 37},
        {(int)QStyle::PM_IndicatorHeight, 38},
        {(int)QStyle::PM_ExclusiveIndicatorWidth, 39},
        {(int)QStyle::PM_ExclusiveIndicatorHeight, 40},
        {(int)QStyle::PM_DialogButtonsSeparator, 41},
        {(int)QStyle::PM_DialogButtonsButtonWidth, 42},
        {(int)QStyle::PM_DialogButtonsButtonHeight, 43},
        {(int)QStyle::PM_MdiSubWindowFrameWidth, 44},
        {(int)QStyle::PM_MDIFrameWidth, 45},
        {(int)QStyle::PM_MdiSubWindowMinimizedWidth, 46},
        {(int)QStyle::PM_MDIMinimizedWidth, 47},
        {(int)QStyle::PM_HeaderMargin, 48},
        {(int)QStyle::PM_HeaderMarkSize, 49},
        {(int)QStyle::PM_HeaderGripMargin, 50},
        {(int)QStyle::PM_TabBarTabShiftHorizontal, 51},
        {(int)QStyle::PM_TabBarTabShiftVertical, 52},
        {(int)QStyle::PM_TabBarScrollButtonWidth, 53},
        {(int)QStyle::PM_ToolBarFrameWidth, 54},
        {(int)QStyle::PM_ToolBarHandleExtent, 55},
        {(int)QStyle::PM_ToolBarItemSpacing, 56},
        {(int)QStyle::PM_ToolBarItemMargin, 57},
        {(int)QStyle::PM_ToolBarSeparatorExtent, 58},
        {(int)QStyle::PM_ToolBarExtensionExtent, 59},
        {(int)QStyle::PM_SpinBoxSliderHeight, 60},
        {(int)QStyle::PM_DefaultTopLevelMargin, 61},
        {(int)QStyle::PM_DefaultChildMargin, 62},
        {(int)QStyle::PM_DefaultLayoutSpacing, 63},
        {(int)QStyle::PM_ToolBarIconSize, 64},
        {(int)QStyle::PM_ListViewIconSize, 65},
        {(int)QStyle::PM_IconViewIconSize, 66},
        {(int)QStyle::PM_SmallIconSize, 67},
        {(int)QStyle::PM_LargeIconSize, 68},
        {(int)QStyle::PM_FocusFrameVMargin, 69},
        {(int)QStyle::PM_FocusFrameHMargin, 70},
        {(int)QStyle::PM_ToolTipLabelFrameWidth, 71},
        {(int)QStyle::PM_CheckBoxLabelSpacing, 72},
        {(int)QStyle::PM_TabBarIconSize, 73},
        {(int)QStyle::PM_SizeGripSize, 74},
        {(int)QStyle::PM_DockWidgetTitleMargin, 75},
        {(int)QStyle::PM_MessageBoxIconSize, 76},
        {(int)QStyle::PM_ButtonIconSize, 77},
        {(int)QStyle::PM_DockWidgetTitleBarButtonMargin, 78},
        {(int)QStyle::PM_RadioButtonLabelSpacing, 79},
        {(int)QStyle::PM_LayoutLeftMargin, 80},
        {(int)QStyle::PM_LayoutTopMargin, 81},
        {(int)QStyle::PM_LayoutRightMargin, 82},
        {(int)QStyle::PM_LayoutBottomMargin, 83},
        {(int)QStyle::PM_LayoutHorizontalSpacing, 84},
        {(int)QStyle::PM_LayoutVerticalSpacing, 85},
        {(int)QStyle::PM_TabBar_ScrollButtonOverlap, 86},
        {(int)QStyle::PM_TextCursorWidth, 87},
        {(int)QStyle::PM_TabCloseIndicatorWidth, 88},
        {(int)QStyle::PM_TabCloseIndicatorHeight, 89},
        {(int)QStyle::PM_ScrollView_ScrollBarSpacing, 90},
        {(int)QStyle::PM_ScrollView_ScrollBarOverlap, 91},
        {(int)QStyle::PM_SubMenuOverlap, 92},
        {(int)QStyle::PM_TreeViewIndentation, 93},
        {(int)QStyle::PM_HeaderDefaultSectionSizeHorizontal, 94},
        {(int)QStyle::PM_HeaderDefaultSectionSizeVertical, 95},
        {(int)QStyle::PM_TitleBarButtonIconSize, 96},
        {(int)QStyle::PM_TitleBarButtonSize, 97},
        {(int)QStyle::PM_CustomBase, 98},
    };

    for (int i = 0; i < 99; ++i) {
        s_pixel_metric_lookup[pixel_metric_vals[i].key] =
            pixel_metric_vals[i].val;
    }

    static KeyVal pixmap_fragment_hint_vals[] = {
        {(int)QPainter::OpaqueHint, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_pixmap_fragment_hint_lookup[pixmap_fragment_hint_vals[i].key] =
            pixmap_fragment_hint_vals[i].val;
    }

    static KeyVal pointer_type_vals[] = {
        {(int)QTabletEvent::UnknownPointer, 0},
        {(int)QTabletEvent::Pen, 1},
        {(int)QTabletEvent::Cursor, 2},
        {(int)QTabletEvent::Eraser, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_pointer_type_lookup[pointer_type_vals[i].key] =
            pointer_type_vals[i].val;
    }

    static KeyVal policy_vals[] = {
        {(int)QSizePolicy::Fixed, 0},
        {(int)QSizePolicy::Minimum, 1},
        {(int)QSizePolicy::Maximum, 2},
        {(int)QSizePolicy::Preferred, 3},
        {(int)QSizePolicy::MinimumExpanding, 4},
        {(int)QSizePolicy::Expanding, 5},
        {(int)QSizePolicy::Ignored, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_policy_lookup[policy_vals[i].key] = policy_vals[i].val;
    }

    static KeyVal policy_flag_vals[] = {
        {(int)QSizePolicy::GrowFlag, 0},
        {(int)QSizePolicy::ExpandFlag, 1},
        {(int)QSizePolicy::ShrinkFlag, 2},
        {(int)QSizePolicy::IgnoreFlag, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_policy_flag_lookup[policy_flag_vals[i].key] = policy_flag_vals[i].val;
    }

    static KeyVal polygon_draw_mode_vals[] = {
        {(int)QPaintEngine::OddEvenMode, 0},
        {(int)QPaintEngine::WindingMode, 1},
        {(int)QPaintEngine::ConvexMode, 2},
        {(int)QPaintEngine::PolylineMode, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_polygon_draw_mode_lookup[polygon_draw_mode_vals[i].key] =
            polygon_draw_mode_vals[i].val;
    }

    static KeyVal primitive_element_vals[] = {
        {(int)QStyle::PE_Frame, 0},
        {(int)QStyle::PE_FrameDefaultButton, 1},
        {(int)QStyle::PE_FrameDockWidget, 2},
        {(int)QStyle::PE_FrameFocusRect, 3},
        {(int)QStyle::PE_FrameGroupBox, 4},
        {(int)QStyle::PE_FrameLineEdit, 5},
        {(int)QStyle::PE_FrameMenu, 6},
        {(int)QStyle::PE_FrameStatusBar, 7},
        {(int)QStyle::PE_FrameStatusBarItem, 8},
        {(int)QStyle::PE_FrameTabWidget, 9},
        {(int)QStyle::PE_FrameWindow, 10},
        {(int)QStyle::PE_FrameButtonBevel, 11},
        {(int)QStyle::PE_FrameButtonTool, 12},
        {(int)QStyle::PE_FrameTabBarBase, 13},
        {(int)QStyle::PE_PanelButtonCommand, 14},
        {(int)QStyle::PE_PanelButtonBevel, 15},
        {(int)QStyle::PE_PanelButtonTool, 16},
        {(int)QStyle::PE_PanelMenuBar, 17},
        {(int)QStyle::PE_PanelToolBar, 18},
        {(int)QStyle::PE_PanelLineEdit, 19},
        {(int)QStyle::PE_IndicatorArrowDown, 20},
        {(int)QStyle::PE_IndicatorArrowLeft, 21},
        {(int)QStyle::PE_IndicatorArrowRight, 22},
        {(int)QStyle::PE_IndicatorArrowUp, 23},
        {(int)QStyle::PE_IndicatorBranch, 24},
        {(int)QStyle::PE_IndicatorButtonDropDown, 25},
        {(int)QStyle::PE_IndicatorViewItemCheck, 26},
        {(int)QStyle::PE_IndicatorItemViewItemCheck, 27},
        {(int)QStyle::PE_IndicatorCheckBox, 28},
        {(int)QStyle::PE_IndicatorDockWidgetResizeHandle, 29},
        {(int)QStyle::PE_IndicatorHeaderArrow, 30},
        {(int)QStyle::PE_IndicatorMenuCheckMark, 31},
        {(int)QStyle::PE_IndicatorProgressChunk, 32},
        {(int)QStyle::PE_IndicatorRadioButton, 33},
        {(int)QStyle::PE_IndicatorSpinDown, 34},
        {(int)QStyle::PE_IndicatorSpinMinus, 35},
        {(int)QStyle::PE_IndicatorSpinPlus, 36},
        {(int)QStyle::PE_IndicatorSpinUp, 37},
        {(int)QStyle::PE_IndicatorToolBarHandle, 38},
        {(int)QStyle::PE_IndicatorToolBarSeparator, 39},
        {(int)QStyle::PE_PanelTipLabel, 40},
        {(int)QStyle::PE_IndicatorTabTear, 41},
        {(int)QStyle::PE_IndicatorTabTearLeft, 42},
        {(int)QStyle::PE_PanelScrollAreaCorner, 43},
        {(int)QStyle::PE_Widget, 44},
        {(int)QStyle::PE_IndicatorColumnViewArrow, 45},
        {(int)QStyle::PE_IndicatorItemViewItemDrop, 46},
        {(int)QStyle::PE_PanelItemViewItem, 47},
        {(int)QStyle::PE_PanelItemViewRow, 48},
        {(int)QStyle::PE_PanelStatusBar, 49},
        {(int)QStyle::PE_IndicatorTabClose, 50},
        {(int)QStyle::PE_PanelMenu, 51},
        {(int)QStyle::PE_IndicatorTabTearRight, 52},
        {(int)QStyle::PE_CustomBase, 53},
    };

    for (int i = 0; i < 54; ++i) {
        s_primitive_element_lookup[primitive_element_vals[i].key] =
            primitive_element_vals[i].val;
    }

    static KeyVal reason_vals[] = {
        {(int)QContextMenuEvent::Mouse, 0},
        {(int)QContextMenuEvent::Keyboard, 1},
        {(int)QContextMenuEvent::Other, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_reason_lookup[reason_vals[i].key] = reason_vals[i].val;
    }

    static KeyVal region_type_vals[] = {
        {(int)QRegion::Rectangle, 0},
        {(int)QRegion::Ellipse, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_region_type_lookup[region_type_vals[i].key] = region_type_vals[i].val;
    }

    static KeyVal render_flag_vals[] = {
        {(int)QWidget::DrawWindowBackground, 0},
        {(int)QWidget::DrawChildren, 1},
        {(int)QWidget::IgnoreMask, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_render_flag_lookup[render_flag_vals[i].key] = render_flag_vals[i].val;
    }

    static KeyVal render_hint_vals[] = {
        {(int)QPainter::Antialiasing, 0},
        {(int)QPainter::TextAntialiasing, 1},
        {(int)QPainter::SmoothPixmapTransform, 2},
        {(int)QPainter::HighQualityAntialiasing, 3},
        {(int)QPainter::NonCosmeticDefaultPen, 4},
        {(int)QPainter::Qt4CompatiblePainting, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_render_hint_lookup[render_hint_vals[i].key] = render_hint_vals[i].val;
    }

    static KeyVal renderable_type_vals[] = {
        {(int)QSurfaceFormat::DefaultRenderableType, 0},
        {(int)QSurfaceFormat::OpenGL, 1},
        {(int)QSurfaceFormat::OpenGLES, 2},
        {(int)QSurfaceFormat::OpenVG, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_renderable_type_lookup[renderable_type_vals[i].key] =
            renderable_type_vals[i].val;
    }

    static KeyVal request_software_input_panel_vals[] = {
        {(int)QStyle::RSIP_OnMouseClickAndAlreadyFocused, 0},
        {(int)QStyle::RSIP_OnMouseClick, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_request_software_input_panel_lookup
            [request_software_input_panel_vals[i].key] =
                request_software_input_panel_vals[i].val;
    }

    static KeyVal resolve_properties_vals[] = {
        {(int)QFont::FamilyResolved, 0},
        {(int)QFont::SizeResolved, 1},
        {(int)QFont::StyleHintResolved, 2},
        {(int)QFont::StyleStrategyResolved, 3},
        {(int)QFont::WeightResolved, 4},
        {(int)QFont::StyleResolved, 5},
        {(int)QFont::UnderlineResolved, 6},
        {(int)QFont::OverlineResolved, 7},
        {(int)QFont::StrikeOutResolved, 8},
        {(int)QFont::FixedPitchResolved, 9},
        {(int)QFont::StretchResolved, 10},
        {(int)QFont::KerningResolved, 11},
        {(int)QFont::CapitalizationResolved, 12},
        {(int)QFont::LetterSpacingResolved, 13},
        {(int)QFont::WordSpacingResolved, 14},
        {(int)QFont::HintingPreferenceResolved, 15},
        {(int)QFont::StyleNameResolved, 16},
        {(int)QFont::AllPropertiesResolved, 17},
    };

    for (int i = 0; i < 18; ++i) {
        s_resolve_properties_lookup[resolve_properties_vals[i].key] =
            resolve_properties_vals[i].val;
    }

    static KeyVal screen_orientation_vals[] = {
        {(int)Qt::PrimaryOrientation, 0},
        {(int)Qt::PortraitOrientation, 1},
        {(int)Qt::LandscapeOrientation, 2},
        {(int)Qt::InvertedPortraitOrientation, 3},
        {(int)Qt::InvertedLandscapeOrientation, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_screen_orientation_lookup[screen_orientation_vals[i].key] =
            screen_orientation_vals[i].val;
    }

    static KeyVal scroll_bar_policy_vals[] = {
        {(int)Qt::ScrollBarAsNeeded, 0},
        {(int)Qt::ScrollBarAlwaysOff, 1},
        {(int)Qt::ScrollBarAlwaysOn, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_scroll_bar_policy_lookup[scroll_bar_policy_vals[i].key] =
            scroll_bar_policy_vals[i].val;
    }

    static KeyVal scroll_phase_vals[] = {
        {(int)Qt::NoScrollPhase, 0},
        {(int)Qt::ScrollBegin, 1},
        {(int)Qt::ScrollUpdate, 2},
        {(int)Qt::ScrollEnd, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_scroll_phase_lookup[scroll_phase_vals[i].key] =
            scroll_phase_vals[i].val;
    }

    static KeyVal sequence_format_vals[] = {
        {(int)QKeySequence::NativeText, 0},
        {(int)QKeySequence::PortableText, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_sequence_format_lookup[sequence_format_vals[i].key] =
            sequence_format_vals[i].val;
    }

    static KeyVal sequence_match_vals[] = {
        {(int)QKeySequence::NoMatch, 0},
        {(int)QKeySequence::PartialMatch, 1},
        {(int)QKeySequence::ExactMatch, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_sequence_match_lookup[sequence_match_vals[i].key] =
            sequence_match_vals[i].val;
    }

    static KeyVal shortcut_context_vals[] = {
        {(int)Qt::WidgetShortcut, 0},
        {(int)Qt::WindowShortcut, 1},
        {(int)Qt::ApplicationShortcut, 2},
        {(int)Qt::WidgetWithChildrenShortcut, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_shortcut_context_lookup[shortcut_context_vals[i].key] =
            shortcut_context_vals[i].val;
    }

    static KeyVal size_constraint_vals[] = {
        {(int)QLayout::SetDefaultConstraint, 0},
        {(int)QLayout::SetNoConstraint, 1},
        {(int)QLayout::SetMinimumSize, 2},
        {(int)QLayout::SetFixedSize, 3},
        {(int)QLayout::SetMaximumSize, 4},
        {(int)QLayout::SetMinAndMaxSize, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_size_constraint_lookup[size_constraint_vals[i].key] =
            size_constraint_vals[i].val;
    }

    static KeyVal size_hint_vals[] = {
        {(int)Qt::MinimumSize, 0}, {(int)Qt::PreferredSize, 1},
        {(int)Qt::MaximumSize, 2}, {(int)Qt::MinimumDescent, 3},
        {(int)Qt::NSizeHints, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_size_hint_lookup[size_hint_vals[i].key] = size_hint_vals[i].val;
    }

    static KeyVal size_mode_vals[] = {
        {(int)Qt::AbsoluteSize, 0},
        {(int)Qt::RelativeSize, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_size_mode_lookup[size_mode_vals[i].key] = size_mode_vals[i].val;
    }

    static KeyVal sort_order_vals[] = {
        {(int)Qt::AscendingOrder, 0},
        {(int)Qt::DescendingOrder, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_sort_order_lookup[sort_order_vals[i].key] = sort_order_vals[i].val;
    }

    static KeyVal spacing_type_vals[] = {
        {(int)QFont::PercentageSpacing, 0},
        {(int)QFont::AbsoluteSpacing, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_spacing_type_lookup[spacing_type_vals[i].key] =
            spacing_type_vals[i].val;
    }

    static KeyVal spec_vals[] = {
        {(int)QColor::Invalid, 0}, {(int)QColor::Rgb, 1}, {(int)QColor::Hsv, 2},
        {(int)QColor::Cmyk, 3},    {(int)QColor::Hsl, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_spec_lookup[spec_vals[i].key] = spec_vals[i].val;
    }

    static KeyVal spread_vals[] = {
        {(int)QGradient::PadSpread, 0},
        {(int)QGradient::ReflectSpread, 1},
        {(int)QGradient::RepeatSpread, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_spread_lookup[spread_vals[i].key] = spread_vals[i].val;
    }

    static KeyVal standard_key_vals[] = {
        {(int)QKeySequence::UnknownKey, 0},
        {(int)QKeySequence::HelpContents, 1},
        {(int)QKeySequence::WhatsThis, 2},
        {(int)QKeySequence::Open, 3},
        {(int)QKeySequence::Close, 4},
        {(int)QKeySequence::Save, 5},
        {(int)QKeySequence::New, 6},
        {(int)QKeySequence::Delete, 7},
        {(int)QKeySequence::Cut, 8},
        {(int)QKeySequence::Copy, 9},
        {(int)QKeySequence::Paste, 10},
        {(int)QKeySequence::Undo, 11},
        {(int)QKeySequence::Redo, 12},
        {(int)QKeySequence::Back, 13},
        {(int)QKeySequence::Forward, 14},
        {(int)QKeySequence::Refresh, 15},
        {(int)QKeySequence::ZoomIn, 16},
        {(int)QKeySequence::ZoomOut, 17},
        {(int)QKeySequence::Print, 18},
        {(int)QKeySequence::AddTab, 19},
        {(int)QKeySequence::NextChild, 20},
        {(int)QKeySequence::PreviousChild, 21},
        {(int)QKeySequence::Find, 22},
        {(int)QKeySequence::FindNext, 23},
        {(int)QKeySequence::FindPrevious, 24},
        {(int)QKeySequence::Replace, 25},
        {(int)QKeySequence::SelectAll, 26},
        {(int)QKeySequence::Bold, 27},
        {(int)QKeySequence::Italic, 28},
        {(int)QKeySequence::Underline, 29},
        {(int)QKeySequence::MoveToNextChar, 30},
        {(int)QKeySequence::MoveToPreviousChar, 31},
        {(int)QKeySequence::MoveToNextWord, 32},
        {(int)QKeySequence::MoveToPreviousWord, 33},
        {(int)QKeySequence::MoveToNextLine, 34},
        {(int)QKeySequence::MoveToPreviousLine, 35},
        {(int)QKeySequence::MoveToNextPage, 36},
        {(int)QKeySequence::MoveToPreviousPage, 37},
        {(int)QKeySequence::MoveToStartOfLine, 38},
        {(int)QKeySequence::MoveToEndOfLine, 39},
        {(int)QKeySequence::MoveToStartOfBlock, 40},
        {(int)QKeySequence::MoveToEndOfBlock, 41},
        {(int)QKeySequence::MoveToStartOfDocument, 42},
        {(int)QKeySequence::MoveToEndOfDocument, 43},
        {(int)QKeySequence::SelectNextChar, 44},
        {(int)QKeySequence::SelectPreviousChar, 45},
        {(int)QKeySequence::SelectNextWord, 46},
        {(int)QKeySequence::SelectPreviousWord, 47},
        {(int)QKeySequence::SelectNextLine, 48},
        {(int)QKeySequence::SelectPreviousLine, 49},
        {(int)QKeySequence::SelectNextPage, 50},
        {(int)QKeySequence::SelectPreviousPage, 51},
        {(int)QKeySequence::SelectStartOfLine, 52},
        {(int)QKeySequence::SelectEndOfLine, 53},
        {(int)QKeySequence::SelectStartOfBlock, 54},
        {(int)QKeySequence::SelectEndOfBlock, 55},
        {(int)QKeySequence::SelectStartOfDocument, 56},
        {(int)QKeySequence::SelectEndOfDocument, 57},
        {(int)QKeySequence::DeleteStartOfWord, 58},
        {(int)QKeySequence::DeleteEndOfWord, 59},
        {(int)QKeySequence::DeleteEndOfLine, 60},
        {(int)QKeySequence::InsertParagraphSeparator, 61},
        {(int)QKeySequence::InsertLineSeparator, 62},
        {(int)QKeySequence::SaveAs, 63},
        {(int)QKeySequence::Preferences, 64},
        {(int)QKeySequence::Quit, 65},
        {(int)QKeySequence::FullScreen, 66},
        {(int)QKeySequence::Deselect, 67},
        {(int)QKeySequence::DeleteCompleteLine, 68},
        {(int)QKeySequence::Backspace, 69},
        {(int)QKeySequence::Cancel, 70},
    };

    for (int i = 0; i < 71; ++i) {
        s_standard_key_lookup[standard_key_vals[i].key] =
            standard_key_vals[i].val;
    }

    static KeyVal standard_pixmap_vals[] = {
        {(int)QStyle::SP_TitleBarMenuButton, 0},
        {(int)QStyle::SP_TitleBarMinButton, 1},
        {(int)QStyle::SP_TitleBarMaxButton, 2},
        {(int)QStyle::SP_TitleBarCloseButton, 3},
        {(int)QStyle::SP_TitleBarNormalButton, 4},
        {(int)QStyle::SP_TitleBarShadeButton, 5},
        {(int)QStyle::SP_TitleBarUnshadeButton, 6},
        {(int)QStyle::SP_TitleBarContextHelpButton, 7},
        {(int)QStyle::SP_DockWidgetCloseButton, 8},
        {(int)QStyle::SP_MessageBoxInformation, 9},
        {(int)QStyle::SP_MessageBoxWarning, 10},
        {(int)QStyle::SP_MessageBoxCritical, 11},
        {(int)QStyle::SP_MessageBoxQuestion, 12},
        {(int)QStyle::SP_DesktopIcon, 13},
        {(int)QStyle::SP_TrashIcon, 14},
        {(int)QStyle::SP_ComputerIcon, 15},
        {(int)QStyle::SP_DriveFDIcon, 16},
        {(int)QStyle::SP_DriveHDIcon, 17},
        {(int)QStyle::SP_DriveCDIcon, 18},
        {(int)QStyle::SP_DriveDVDIcon, 19},
        {(int)QStyle::SP_DriveNetIcon, 20},
        {(int)QStyle::SP_DirOpenIcon, 21},
        {(int)QStyle::SP_DirClosedIcon, 22},
        {(int)QStyle::SP_DirLinkIcon, 23},
        {(int)QStyle::SP_DirLinkOpenIcon, 24},
        {(int)QStyle::SP_FileIcon, 25},
        {(int)QStyle::SP_FileLinkIcon, 26},
        {(int)QStyle::SP_ToolBarHorizontalExtensionButton, 27},
        {(int)QStyle::SP_ToolBarVerticalExtensionButton, 28},
        {(int)QStyle::SP_FileDialogStart, 29},
        {(int)QStyle::SP_FileDialogEnd, 30},
        {(int)QStyle::SP_FileDialogToParent, 31},
        {(int)QStyle::SP_FileDialogNewFolder, 32},
        {(int)QStyle::SP_FileDialogDetailedView, 33},
        {(int)QStyle::SP_FileDialogInfoView, 34},
        {(int)QStyle::SP_FileDialogContentsView, 35},
        {(int)QStyle::SP_FileDialogListView, 36},
        {(int)QStyle::SP_FileDialogBack, 37},
        {(int)QStyle::SP_DirIcon, 38},
        {(int)QStyle::SP_DialogOkButton, 39},
        {(int)QStyle::SP_DialogCancelButton, 40},
        {(int)QStyle::SP_DialogHelpButton, 41},
        {(int)QStyle::SP_DialogOpenButton, 42},
        {(int)QStyle::SP_DialogSaveButton, 43},
        {(int)QStyle::SP_DialogCloseButton, 44},
        {(int)QStyle::SP_DialogApplyButton, 45},
        {(int)QStyle::SP_DialogResetButton, 46},
        {(int)QStyle::SP_DialogDiscardButton, 47},
        {(int)QStyle::SP_DialogYesButton, 48},
        {(int)QStyle::SP_DialogNoButton, 49},
        {(int)QStyle::SP_ArrowUp, 50},
        {(int)QStyle::SP_ArrowDown, 51},
        {(int)QStyle::SP_ArrowLeft, 52},
        {(int)QStyle::SP_ArrowRight, 53},
        {(int)QStyle::SP_ArrowBack, 54},
        {(int)QStyle::SP_ArrowForward, 55},
        {(int)QStyle::SP_DirHomeIcon, 56},
        {(int)QStyle::SP_CommandLink, 57},
        {(int)QStyle::SP_VistaShield, 58},
        {(int)QStyle::SP_BrowserReload, 59},
        {(int)QStyle::SP_BrowserStop, 60},
        {(int)QStyle::SP_MediaPlay, 61},
        {(int)QStyle::SP_MediaStop, 62},
        {(int)QStyle::SP_MediaPause, 63},
        {(int)QStyle::SP_MediaSkipForward, 64},
        {(int)QStyle::SP_MediaSkipBackward, 65},
        {(int)QStyle::SP_MediaSeekForward, 66},
        {(int)QStyle::SP_MediaSeekBackward, 67},
        {(int)QStyle::SP_MediaVolume, 68},
        {(int)QStyle::SP_MediaVolumeMuted, 69},
        {(int)QStyle::SP_LineEditClearButton, 70},
        {(int)QStyle::SP_CustomBase, 71},
    };

    for (int i = 0; i < 72; ++i) {
        s_standard_pixmap_lookup[standard_pixmap_vals[i].key] =
            standard_pixmap_vals[i].val;
    }

    static KeyVal state_vals[] = {
        {(int)QIcon::On, 0},
        {(int)QIcon::Off, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_state_lookup[state_vals[i].key] = state_vals[i].val;
    }

    static KeyVal state_flag_vals[] = {
        {(int)QStyle::State_None, 0},
        {(int)QStyle::State_Enabled, 1},
        {(int)QStyle::State_Raised, 2},
        {(int)QStyle::State_Sunken, 3},
        {(int)QStyle::State_Off, 4},
        {(int)QStyle::State_NoChange, 5},
        {(int)QStyle::State_On, 6},
        {(int)QStyle::State_DownArrow, 7},
        {(int)QStyle::State_Horizontal, 8},
        {(int)QStyle::State_HasFocus, 9},
        {(int)QStyle::State_Top, 10},
        {(int)QStyle::State_Bottom, 11},
        {(int)QStyle::State_FocusAtBorder, 12},
        {(int)QStyle::State_AutoRaise, 13},
        {(int)QStyle::State_MouseOver, 14},
        {(int)QStyle::State_UpArrow, 15},
        {(int)QStyle::State_Selected, 16},
        {(int)QStyle::State_Active, 17},
        {(int)QStyle::State_Window, 18},
        {(int)QStyle::State_Open, 19},
        {(int)QStyle::State_Children, 20},
        {(int)QStyle::State_Item, 21},
        {(int)QStyle::State_Sibling, 22},
        {(int)QStyle::State_Editing, 23},
        {(int)QStyle::State_KeyboardFocusChange, 24},
        {(int)QStyle::State_ReadOnly, 25},
        {(int)QStyle::State_Small, 26},
        {(int)QStyle::State_Mini, 27},
    };

    for (int i = 0; i < 28; ++i) {
        s_state_flag_lookup[state_flag_vals[i].key] = state_flag_vals[i].val;
    }

    static KeyVal stretch_vals[] = {
        {(int)QFont::AnyStretch, 0},     {(int)QFont::UltraCondensed, 1},
        {(int)QFont::ExtraCondensed, 2}, {(int)QFont::Condensed, 3},
        {(int)QFont::SemiCondensed, 4},  {(int)QFont::Unstretched, 5},
        {(int)QFont::SemiExpanded, 6},   {(int)QFont::Expanded, 7},
        {(int)QFont::ExtraExpanded, 8},  {(int)QFont::UltraExpanded, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_stretch_lookup[stretch_vals[i].key] = stretch_vals[i].val;
    }

    static KeyVal style_hint_vals[] = {
        {(int)QStyle::SH_EtchDisabledText, 0},
        {(int)QStyle::SH_DitherDisabledText, 1},
        {(int)QStyle::SH_ScrollBar_MiddleClickAbsolutePosition, 2},
        {(int)QStyle::SH_ScrollBar_ScrollWhenPointerLeavesControl, 3},
        {(int)QStyle::SH_TabBar_SelectMouseType, 4},
        {(int)QStyle::SH_TabBar_Alignment, 5},
        {(int)QStyle::SH_Header_ArrowAlignment, 6},
        {(int)QStyle::SH_Slider_SnapToValue, 7},
        {(int)QStyle::SH_Slider_SloppyKeyEvents, 8},
        {(int)QStyle::SH_ProgressDialog_CenterCancelButton, 9},
        {(int)QStyle::SH_ProgressDialog_TextLabelAlignment, 10},
        {(int)QStyle::SH_PrintDialog_RightAlignButtons, 11},
        {(int)QStyle::SH_MainWindow_SpaceBelowMenuBar, 12},
        {(int)QStyle::SH_FontDialog_SelectAssociatedText, 13},
        {(int)QStyle::SH_Menu_AllowActiveAndDisabled, 14},
        {(int)QStyle::SH_Menu_SpaceActivatesItem, 15},
        {(int)QStyle::SH_Menu_SubMenuPopupDelay, 16},
        {(int)QStyle::SH_ScrollView_FrameOnlyAroundContents, 17},
        {(int)QStyle::SH_MenuBar_AltKeyNavigation, 18},
        {(int)QStyle::SH_ComboBox_ListMouseTracking, 19},
        {(int)QStyle::SH_Menu_MouseTracking, 20},
        {(int)QStyle::SH_MenuBar_MouseTracking, 21},
        {(int)QStyle::SH_ItemView_ChangeHighlightOnFocus, 22},
        {(int)QStyle::SH_Widget_ShareActivation, 23},
        {(int)QStyle::SH_Workspace_FillSpaceOnMaximize, 24},
        {(int)QStyle::SH_ComboBox_Popup, 25},
        {(int)QStyle::SH_TitleBar_NoBorder, 26},
        {(int)QStyle::SH_Slider_StopMouseOverSlider, 27},
        {(int)QStyle::SH_ScrollBar_StopMouseOverSlider, 28},
        {(int)QStyle::SH_BlinkCursorWhenTextSelected, 29},
        {(int)QStyle::SH_RichText_FullWidthSelection, 30},
        {(int)QStyle::SH_Menu_Scrollable, 31},
        {(int)QStyle::SH_GroupBox_TextLabelVerticalAlignment, 32},
        {(int)QStyle::SH_GroupBox_TextLabelColor, 33},
        {(int)QStyle::SH_Menu_SloppySubMenus, 34},
        {(int)QStyle::SH_Table_GridLineColor, 35},
        {(int)QStyle::SH_LineEdit_PasswordCharacter, 36},
        {(int)QStyle::SH_DialogButtons_DefaultButton, 37},
        {(int)QStyle::SH_ToolBox_SelectedPageTitleBold, 38},
        {(int)QStyle::SH_TabBar_PreferNoArrows, 39},
        {(int)QStyle::SH_ScrollBar_LeftClickAbsolutePosition, 40},
        {(int)QStyle::SH_ListViewExpand_SelectMouseType, 41},
        {(int)QStyle::SH_UnderlineShortcut, 42},
        {(int)QStyle::SH_SpinBox_AnimateButton, 43},
        {(int)QStyle::SH_SpinBox_KeyPressAutoRepeatRate, 44},
        {(int)QStyle::SH_SpinBox_ClickAutoRepeatRate, 45},
        {(int)QStyle::SH_Menu_FillScreenWithScroll, 46},
        {(int)QStyle::SH_ToolTipLabel_Opacity, 47},
        {(int)QStyle::SH_DrawMenuBarSeparator, 48},
        {(int)QStyle::SH_TitleBar_ModifyNotification, 49},
        {(int)QStyle::SH_Button_FocusPolicy, 50},
        {(int)QStyle::SH_MessageBox_UseBorderForButtonSpacing, 51},
        {(int)QStyle::SH_TitleBar_AutoRaise, 52},
        {(int)QStyle::SH_ToolButton_PopupDelay, 53},
        {(int)QStyle::SH_FocusFrame_Mask, 54},
        {(int)QStyle::SH_RubberBand_Mask, 55},
        {(int)QStyle::SH_WindowFrame_Mask, 56},
        {(int)QStyle::SH_SpinControls_DisableOnBounds, 57},
        {(int)QStyle::SH_Dial_BackgroundRole, 58},
        {(int)QStyle::SH_ComboBox_LayoutDirection, 59},
        {(int)QStyle::SH_ItemView_EllipsisLocation, 60},
        {(int)QStyle::SH_ItemView_ShowDecorationSelected, 61},
        {(int)QStyle::SH_ItemView_ActivateItemOnSingleClick, 62},
        {(int)QStyle::SH_ScrollBar_ContextMenu, 63},
        {(int)QStyle::SH_ScrollBar_RollBetweenButtons, 64},
        {(int)QStyle::SH_Slider_AbsoluteSetButtons, 65},
        {(int)QStyle::SH_Slider_PageSetButtons, 66},
        {(int)QStyle::SH_Menu_KeyboardSearch, 67},
        {(int)QStyle::SH_TabBar_ElideMode, 68},
        {(int)QStyle::SH_DialogButtonLayout, 69},
        {(int)QStyle::SH_ComboBox_PopupFrameStyle, 70},
        {(int)QStyle::SH_MessageBox_TextInteractionFlags, 71},
        {(int)QStyle::SH_DialogButtonBox_ButtonsHaveIcons, 72},
        {(int)QStyle::SH_SpellCheckUnderlineStyle, 73},
        {(int)QStyle::SH_MessageBox_CenterButtons, 74},
        {(int)QStyle::SH_Menu_SelectionWrap, 75},
        {(int)QStyle::SH_ItemView_MovementWithoutUpdatingSelection, 76},
        {(int)QStyle::SH_ToolTip_Mask, 77},
        {(int)QStyle::SH_FocusFrame_AboveWidget, 78},
        {(int)QStyle::SH_TextControl_FocusIndicatorTextCharFormat, 79},
        {(int)QStyle::SH_WizardStyle, 80},
        {(int)QStyle::SH_ItemView_ArrowKeysNavigateIntoChildren, 81},
        {(int)QStyle::SH_Menu_Mask, 82},
        {(int)QStyle::SH_Menu_FlashTriggeredItem, 83},
        {(int)QStyle::SH_Menu_FadeOutOnHide, 84},
        {(int)QStyle::SH_SpinBox_ClickAutoRepeatThreshold, 85},
        {(int)QStyle::SH_ItemView_PaintAlternatingRowColorsForEmptyArea, 86},
        {(int)QStyle::SH_FormLayoutWrapPolicy, 87},
        {(int)QStyle::SH_TabWidget_DefaultTabPosition, 88},
        {(int)QStyle::SH_ToolBar_Movable, 89},
        {(int)QStyle::SH_FormLayoutFieldGrowthPolicy, 90},
        {(int)QStyle::SH_FormLayoutFormAlignment, 91},
        {(int)QStyle::SH_FormLayoutLabelAlignment, 92},
        {(int)QStyle::SH_ItemView_DrawDelegateFrame, 93},
        {(int)QStyle::SH_TabBar_CloseButtonPosition, 94},
        {(int)QStyle::SH_DockWidget_ButtonsHaveFrame, 95},
        {(int)QStyle::SH_ToolButtonStyle, 96},
        {(int)QStyle::SH_RequestSoftwareInputPanel, 97},
        {(int)QStyle::SH_ScrollBar_Transient, 98},
        {(int)QStyle::SH_Menu_SupportsSections, 99},
        {(int)QStyle::SH_ToolTip_WakeUpDelay, 100},
        {(int)QStyle::SH_ToolTip_FallAsleepDelay, 101},
        {(int)QStyle::SH_Widget_Animate, 102},
        {(int)QStyle::SH_Splitter_OpaqueResize, 103},
        {(int)QStyle::SH_ComboBox_UseNativePopup, 104},
        {(int)QStyle::SH_LineEdit_PasswordMaskDelay, 105},
        {(int)QStyle::SH_TabBar_ChangeCurrentDelay, 106},
        {(int)QStyle::SH_Menu_SubMenuUniDirection, 107},
        {(int)QStyle::SH_Menu_SubMenuUniDirectionFailCount, 108},
        {(int)QStyle::SH_Menu_SubMenuSloppySelectOtherActions, 109},
        {(int)QStyle::SH_Menu_SubMenuSloppyCloseTimeout, 110},
        {(int)QStyle::SH_Menu_SubMenuResetWhenReenteringParent, 111},
        {(int)QStyle::SH_Menu_SubMenuDontStartSloppyOnLeave, 112},
        {(int)QStyle::SH_ItemView_ScrollMode, 113},
        {(int)QStyle::SH_TitleBar_ShowToolTipsOnButtons, 114},
        {(int)QStyle::SH_Widget_Animation_Duration, 115},
        {(int)QStyle::SH_ComboBox_AllowWheelScrolling, 116},
        {(int)QStyle::SH_SpinBox_ButtonsInsideFrame, 117},
        {(int)QStyle::SH_CustomBase, 118},
    };

    for (int i = 0; i < 119; ++i) {
        s_style_hint_lookup[style_hint_vals[i].key] = style_hint_vals[i].val;
    }

    static KeyVal style_strategy_vals[] = {
        {(int)QFont::PreferDefault, 0},
        {(int)QFont::PreferBitmap, 1},
        {(int)QFont::PreferDevice, 2},
        {(int)QFont::PreferOutline, 3},
        {(int)QFont::ForceOutline, 4},
        {(int)QFont::PreferMatch, 5},
        {(int)QFont::PreferQuality, 6},
        {(int)QFont::PreferAntialias, 7},
        {(int)QFont::NoAntialias, 8},
        {(int)QFont::OpenGLCompatible, 9},
        {(int)QFont::ForceIntegerMetrics, 10},
        {(int)QFont::NoSubpixelAntialias, 11},
        {(int)QFont::PreferNoShaping, 12},
        {(int)QFont::NoFontMerging, 13},
    };

    for (int i = 0; i < 14; ++i) {
        s_style_strategy_lookup[style_strategy_vals[i].key] =
            style_strategy_vals[i].val;
    }

    static KeyVal sub_control_vals[] = {
        {(int)QStyle::SC_None, 0},
        {(int)QStyle::SC_ScrollBarAddLine, 1},
        {(int)QStyle::SC_ScrollBarSubLine, 2},
        {(int)QStyle::SC_ScrollBarAddPage, 3},
        {(int)QStyle::SC_ScrollBarSubPage, 4},
        {(int)QStyle::SC_ScrollBarFirst, 5},
        {(int)QStyle::SC_ScrollBarLast, 6},
        {(int)QStyle::SC_ScrollBarSlider, 7},
        {(int)QStyle::SC_ScrollBarGroove, 8},
        {(int)QStyle::SC_SpinBoxUp, 9},
        {(int)QStyle::SC_SpinBoxDown, 10},
        {(int)QStyle::SC_SpinBoxFrame, 11},
        {(int)QStyle::SC_SpinBoxEditField, 12},
        {(int)QStyle::SC_ComboBoxFrame, 13},
        {(int)QStyle::SC_ComboBoxEditField, 14},
        {(int)QStyle::SC_ComboBoxArrow, 15},
        {(int)QStyle::SC_ComboBoxListBoxPopup, 16},
        {(int)QStyle::SC_SliderGroove, 17},
        {(int)QStyle::SC_SliderHandle, 18},
        {(int)QStyle::SC_SliderTickmarks, 19},
        {(int)QStyle::SC_ToolButton, 20},
        {(int)QStyle::SC_ToolButtonMenu, 21},
        {(int)QStyle::SC_TitleBarSysMenu, 22},
        {(int)QStyle::SC_TitleBarMinButton, 23},
        {(int)QStyle::SC_TitleBarMaxButton, 24},
        {(int)QStyle::SC_TitleBarCloseButton, 25},
        {(int)QStyle::SC_TitleBarNormalButton, 26},
        {(int)QStyle::SC_TitleBarShadeButton, 27},
        {(int)QStyle::SC_TitleBarUnshadeButton, 28},
        {(int)QStyle::SC_TitleBarContextHelpButton, 29},
        {(int)QStyle::SC_TitleBarLabel, 30},
        {(int)QStyle::SC_DialGroove, 31},
        {(int)QStyle::SC_DialHandle, 32},
        {(int)QStyle::SC_DialTickmarks, 33},
        {(int)QStyle::SC_GroupBoxCheckBox, 34},
        {(int)QStyle::SC_GroupBoxLabel, 35},
        {(int)QStyle::SC_GroupBoxContents, 36},
        {(int)QStyle::SC_GroupBoxFrame, 37},
        {(int)QStyle::SC_MdiMinButton, 38},
        {(int)QStyle::SC_MdiNormalButton, 39},
        {(int)QStyle::SC_MdiCloseButton, 40},
        {(int)QStyle::SC_CustomBase, 41},
        {(int)QStyle::SC_All, 42},
    };

    for (int i = 0; i < 43; ++i) {
        s_sub_control_lookup[sub_control_vals[i].key] = sub_control_vals[i].val;
    }

    static KeyVal sub_element_vals[] = {
        {(int)QStyle::SE_PushButtonContents, 0},
        {(int)QStyle::SE_PushButtonFocusRect, 1},
        {(int)QStyle::SE_CheckBoxIndicator, 2},
        {(int)QStyle::SE_CheckBoxContents, 3},
        {(int)QStyle::SE_CheckBoxFocusRect, 4},
        {(int)QStyle::SE_CheckBoxClickRect, 5},
        {(int)QStyle::SE_RadioButtonIndicator, 6},
        {(int)QStyle::SE_RadioButtonContents, 7},
        {(int)QStyle::SE_RadioButtonFocusRect, 8},
        {(int)QStyle::SE_RadioButtonClickRect, 9},
        {(int)QStyle::SE_ComboBoxFocusRect, 10},
        {(int)QStyle::SE_SliderFocusRect, 11},
        {(int)QStyle::SE_ProgressBarGroove, 12},
        {(int)QStyle::SE_ProgressBarContents, 13},
        {(int)QStyle::SE_ProgressBarLabel, 14},
        {(int)QStyle::SE_ToolBoxTabContents, 15},
        {(int)QStyle::SE_HeaderLabel, 16},
        {(int)QStyle::SE_HeaderArrow, 17},
        {(int)QStyle::SE_TabWidgetTabBar, 18},
        {(int)QStyle::SE_TabWidgetTabPane, 19},
        {(int)QStyle::SE_TabWidgetTabContents, 20},
        {(int)QStyle::SE_TabWidgetLeftCorner, 21},
        {(int)QStyle::SE_TabWidgetRightCorner, 22},
        {(int)QStyle::SE_ViewItemCheckIndicator, 23},
        {(int)QStyle::SE_ItemViewItemCheckIndicator, 24},
        {(int)QStyle::SE_TabBarTearIndicator, 25},
        {(int)QStyle::SE_TabBarTearIndicatorLeft, 26},
        {(int)QStyle::SE_TreeViewDisclosureItem, 27},
        {(int)QStyle::SE_LineEditContents, 28},
        {(int)QStyle::SE_FrameContents, 29},
        {(int)QStyle::SE_DockWidgetCloseButton, 30},
        {(int)QStyle::SE_DockWidgetFloatButton, 31},
        {(int)QStyle::SE_DockWidgetTitleBarText, 32},
        {(int)QStyle::SE_DockWidgetIcon, 33},
        {(int)QStyle::SE_CheckBoxLayoutItem, 34},
        {(int)QStyle::SE_ComboBoxLayoutItem, 35},
        {(int)QStyle::SE_DateTimeEditLayoutItem, 36},
        {(int)QStyle::SE_DialogButtonBoxLayoutItem, 37},
        {(int)QStyle::SE_LabelLayoutItem, 38},
        {(int)QStyle::SE_ProgressBarLayoutItem, 39},
        {(int)QStyle::SE_PushButtonLayoutItem, 40},
        {(int)QStyle::SE_RadioButtonLayoutItem, 41},
        {(int)QStyle::SE_SliderLayoutItem, 42},
        {(int)QStyle::SE_SpinBoxLayoutItem, 43},
        {(int)QStyle::SE_ToolButtonLayoutItem, 44},
        {(int)QStyle::SE_FrameLayoutItem, 45},
        {(int)QStyle::SE_GroupBoxLayoutItem, 46},
        {(int)QStyle::SE_TabWidgetLayoutItem, 47},
        {(int)QStyle::SE_ItemViewItemDecoration, 48},
        {(int)QStyle::SE_ItemViewItemText, 49},
        {(int)QStyle::SE_ItemViewItemFocusRect, 50},
        {(int)QStyle::SE_TabBarTabLeftButton, 51},
        {(int)QStyle::SE_TabBarTabRightButton, 52},
        {(int)QStyle::SE_TabBarTabText, 53},
        {(int)QStyle::SE_ShapedFrameContents, 54},
        {(int)QStyle::SE_ToolBarHandle, 55},
        {(int)QStyle::SE_TabBarScrollLeftButton, 56},
        {(int)QStyle::SE_TabBarScrollRightButton, 57},
        {(int)QStyle::SE_TabBarTearIndicatorRight, 58},
        {(int)QStyle::SE_CustomBase, 59},
    };

    for (int i = 0; i < 60; ++i) {
        s_sub_element_lookup[sub_element_vals[i].key] = sub_element_vals[i].val;
    }

    static KeyVal surface_class_vals[] = {
        {(int)QSurface::Window, 0},
        {(int)QSurface::Offscreen, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_surface_class_lookup[surface_class_vals[i].key] =
            surface_class_vals[i].val;
    }

    static KeyVal surface_type_vals[] = {
        {(int)QSurface::RasterSurface, 0},   {(int)QSurface::OpenGLSurface, 1},
        {(int)QSurface::RasterGLSurface, 2}, {(int)QSurface::OpenVGSurface, 3},
        {(int)QSurface::VulkanSurface, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_surface_type_lookup[surface_type_vals[i].key] =
            surface_type_vals[i].val;
    }

    static KeyVal swap_behavior_vals[] = {
        {(int)QSurfaceFormat::DefaultSwapBehavior, 0},
        {(int)QSurfaceFormat::SingleBuffer, 1},
        {(int)QSurfaceFormat::DoubleBuffer, 2},
        {(int)QSurfaceFormat::TripleBuffer, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_swap_behavior_lookup[swap_behavior_vals[i].key] =
            swap_behavior_vals[i].val;
    }

    static KeyVal tab_focus_behavior_vals[] = {
        {(int)Qt::NoTabFocus, 0},
        {(int)Qt::TabFocusTextControls, 1},
        {(int)Qt::TabFocusListControls, 2},
        {(int)Qt::TabFocusAllControls, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_tab_focus_behavior_lookup[tab_focus_behavior_vals[i].key] =
            tab_focus_behavior_vals[i].val;
    }

    static KeyVal tablet_device_vals[] = {
        {(int)QTabletEvent::NoDevice, 0},
        {(int)QTabletEvent::Puck, 1},
        {(int)QTabletEvent::Stylus, 2},
        {(int)QTabletEvent::Airbrush, 3},
        {(int)QTabletEvent::FourDMouse, 4},
        {(int)QTabletEvent::XFreeEraser, 5},
        {(int)QTabletEvent::RotationStylus, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_tablet_device_lookup[tablet_device_vals[i].key] =
            tablet_device_vals[i].val;
    }

    static KeyVal text_elide_mode_vals[] = {
        {(int)Qt::ElideLeft, 0},
        {(int)Qt::ElideRight, 1},
        {(int)Qt::ElideMiddle, 2},
        {(int)Qt::ElideNone, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_text_elide_mode_lookup[text_elide_mode_vals[i].key] =
            text_elide_mode_vals[i].val;
    }

    static KeyVal text_flag_vals[] = {
        {(int)Qt::TextSingleLine, 0},
        {(int)Qt::TextDontClip, 1},
        {(int)Qt::TextExpandTabs, 2},
        {(int)Qt::TextShowMnemonic, 3},
        {(int)Qt::TextWordWrap, 4},
        {(int)Qt::TextWrapAnywhere, 5},
        {(int)Qt::TextDontPrint, 6},
        {(int)Qt::TextIncludeTrailingSpaces, 7},
        {(int)Qt::TextHideMnemonic, 8},
        {(int)Qt::TextJustificationForced, 9},
        {(int)Qt::TextForceLeftToRight, 10},
        {(int)Qt::TextForceRightToLeft, 11},
        {(int)Qt::TextLongestVariant, 12},
        {(int)Qt::TextBypassShaping, 13},
    };

    for (int i = 0; i < 14; ++i) {
        s_text_flag_lookup[text_flag_vals[i].key] = text_flag_vals[i].val;
    }

    static KeyVal text_format_vals[] = {
        {(int)Qt::PlainText, 0},
        {(int)Qt::RichText, 1},
        {(int)Qt::AutoText, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_text_format_lookup[text_format_vals[i].key] = text_format_vals[i].val;
    }

    static KeyVal text_interaction_flag_vals[] = {
        {(int)Qt::NoTextInteraction, 0},
        {(int)Qt::TextSelectableByMouse, 1},
        {(int)Qt::TextSelectableByKeyboard, 2},
        {(int)Qt::LinksAccessibleByMouse, 3},
        {(int)Qt::LinksAccessibleByKeyboard, 4},
        {(int)Qt::TextEditable, 5},
        {(int)Qt::TextEditorInteraction, 6},
        {(int)Qt::TextBrowserInteraction, 7},
    };

    for (int i = 0; i < 8; ++i) {
        s_text_interaction_flag_lookup[text_interaction_flag_vals[i].key] =
            text_interaction_flag_vals[i].val;
    }

    static KeyVal tile_rule_vals[] = {
        {(int)Qt::StretchTile, 0},
        {(int)Qt::RepeatTile, 1},
        {(int)Qt::RoundTile, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_tile_rule_lookup[tile_rule_vals[i].key] = tile_rule_vals[i].val;
    }

    static KeyVal time_spec_vals[] = {
        {(int)Qt::LocalTime, 0},
        {(int)Qt::UTC, 1},
        {(int)Qt::OffsetFromUTC, 2},
        {(int)Qt::TimeZone, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_time_spec_lookup[time_spec_vals[i].key] = time_spec_vals[i].val;
    }

    static KeyVal timer_type_vals[] = {
        {(int)Qt::PreciseTimer, 0},
        {(int)Qt::CoarseTimer, 1},
        {(int)Qt::VeryCoarseTimer, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_timer_type_lookup[timer_type_vals[i].key] = timer_type_vals[i].val;
    }

    static KeyVal tool_bar_area_vals[] = {
        {(int)Qt::LeftToolBarArea, 0},  {(int)Qt::RightToolBarArea, 1},
        {(int)Qt::TopToolBarArea, 2},   {(int)Qt::BottomToolBarArea, 3},
        {(int)Qt::ToolBarArea_Mask, 4}, {(int)Qt::AllToolBarAreas, 5},
        {(int)Qt::NoToolBarArea, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_tool_bar_area_lookup[tool_bar_area_vals[i].key] =
            tool_bar_area_vals[i].val;
    }

    static KeyVal tool_bar_area_sizes_vals[] = {
        {(int)Qt::NToolBarAreas, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_tool_bar_area_sizes_lookup[tool_bar_area_sizes_vals[i].key] =
            tool_bar_area_sizes_vals[i].val;
    }

    static KeyVal tool_button_popup_mode_vals[] = {
        {(int)QToolButton::DelayedPopup, 0},
        {(int)QToolButton::MenuButtonPopup, 1},
        {(int)QToolButton::InstantPopup, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_tool_button_popup_mode_lookup[tool_button_popup_mode_vals[i].key] =
            tool_button_popup_mode_vals[i].val;
    }

    static KeyVal tool_button_style_vals[] = {
        {(int)Qt::ToolButtonIconOnly, 0},
        {(int)Qt::ToolButtonTextOnly, 1},
        {(int)Qt::ToolButtonTextBesideIcon, 2},
        {(int)Qt::ToolButtonTextUnderIcon, 3},
        {(int)Qt::ToolButtonFollowStyle, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_tool_button_style_lookup[tool_button_style_vals[i].key] =
            tool_button_style_vals[i].val;
    }

    static KeyVal touch_point_state_vals[] = {
        {(int)Qt::TouchPointPressed, 0},
        {(int)Qt::TouchPointMoved, 1},
        {(int)Qt::TouchPointStationary, 2},
        {(int)Qt::TouchPointReleased, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_touch_point_state_lookup[touch_point_state_vals[i].key] =
            touch_point_state_vals[i].val;
    }

    static KeyVal transformation_mode_vals[] = {
        {(int)Qt::FastTransformation, 0},
        {(int)Qt::SmoothTransformation, 1},
    };

    for (int i = 0; i < 2; ++i) {
        s_transformation_mode_lookup[transformation_mode_vals[i].key] =
            transformation_mode_vals[i].val;
    }

    static KeyVal transformation_type_vals[] = {
        {(int)QTransform::TxNone, 0},  {(int)QTransform::TxTranslate, 1},
        {(int)QTransform::TxScale, 2}, {(int)QTransform::TxRotate, 3},
        {(int)QTransform::TxShear, 4}, {(int)QTransform::TxProject, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_transformation_type_lookup[transformation_type_vals[i].key] =
            transformation_type_vals[i].val;
    }

    static KeyVal type_vals[] = {
        {(int)QPaintEngine::X11, 0},
        {(int)QPaintEngine::Windows, 1},
        {(int)QPaintEngine::QuickDraw, 2},
        {(int)QPaintEngine::CoreGraphics, 3},
        {(int)QPaintEngine::MacPrinter, 4},
        {(int)QPaintEngine::QWindowSystem, 5},
        {(int)QPaintEngine::PostScript, 6},
        {(int)QPaintEngine::OpenGL, 7},
        {(int)QPaintEngine::Picture, 8},
        {(int)QPaintEngine::SVG, 9},
        {(int)QPaintEngine::Raster, 10},
        {(int)QPaintEngine::Direct3D, 11},
        {(int)QPaintEngine::Pdf, 12},
        {(int)QPaintEngine::OpenVG, 13},
        {(int)QPaintEngine::OpenGL2, 14},
        {(int)QPaintEngine::PaintBuffer, 15},
        {(int)QPaintEngine::Blitter, 16},
        {(int)QPaintEngine::Direct2D, 17},
        {(int)QPaintEngine::User, 18},
        {(int)QPaintEngine::MaxUser, 19},
    };

    for (int i = 0; i < 20; ++i) {
        s_type_lookup[type_vals[i].key] = type_vals[i].val;
    }

    static KeyVal type_interpretation_vals[] = {
        {(int)QPixelFormat::UnsignedInteger, 0},
        {(int)QPixelFormat::UnsignedShort, 1},
        {(int)QPixelFormat::UnsignedByte, 2},
        {(int)QPixelFormat::FloatingPoint, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_type_interpretation_lookup[type_interpretation_vals[i].key] =
            type_interpretation_vals[i].val;
    }

    static KeyVal ui_effect_vals[] = {
        {(int)Qt::UI_General, 0},        {(int)Qt::UI_AnimateMenu, 1},
        {(int)Qt::UI_FadeMenu, 2},       {(int)Qt::UI_AnimateCombo, 3},
        {(int)Qt::UI_AnimateTooltip, 4}, {(int)Qt::UI_FadeTooltip, 5},
        {(int)Qt::UI_AnimateToolBox, 6},
    };

    for (int i = 0; i < 7; ++i) {
        s_ui_effect_lookup[ui_effect_vals[i].key] = ui_effect_vals[i].val;
    }

    static KeyVal visibility_vals[] = {
        {(int)QWindow::Hidden, 0},    {(int)QWindow::AutomaticVisibility, 1},
        {(int)QWindow::Windowed, 2},  {(int)QWindow::Minimized, 3},
        {(int)QWindow::Maximized, 4}, {(int)QWindow::FullScreen, 5},
    };

    for (int i = 0; i < 6; ++i) {
        s_visibility_lookup[visibility_vals[i].key] = visibility_vals[i].val;
    }

    static KeyVal weight_vals[] = {
        {(int)QFont::Thin, 0},   {(int)QFont::ExtraLight, 1},
        {(int)QFont::Light, 2},  {(int)QFont::Normal, 3},
        {(int)QFont::Medium, 4}, {(int)QFont::DemiBold, 5},
        {(int)QFont::Bold, 6},   {(int)QFont::ExtraBold, 7},
        {(int)QFont::Black, 8},
    };

    for (int i = 0; i < 9; ++i) {
        s_weight_lookup[weight_vals[i].key] = weight_vals[i].val;
    }

    static KeyVal wheel_event_fix_me_enums_vals[] = {
        {(int)QWheelEvent::DefaultDeltasPerStep, 0},
    };

    for (int i = 0; i < 1; ++i) {
        s_wheel_event_fix_me_enums_lookup[wheel_event_fix_me_enums_vals[i]
                                              .key] =
            wheel_event_fix_me_enums_vals[i].val;
    }

    static KeyVal white_space_mode_vals[] = {
        {(int)Qt::WhiteSpaceNormal, 0},
        {(int)Qt::WhiteSpacePre, 1},
        {(int)Qt::WhiteSpaceNoWrap, 2},
        {(int)Qt::WhiteSpaceModeUndefined, 3},
    };

    for (int i = 0; i < 4; ++i) {
        s_white_space_mode_lookup[white_space_mode_vals[i].key] =
            white_space_mode_vals[i].val;
    }

    static KeyVal widget_attribute_vals[] = {
        {(int)Qt::WA_Disabled, 0},
        {(int)Qt::WA_UnderMouse, 1},
        {(int)Qt::WA_MouseTracking, 2},
        {(int)Qt::WA_ContentsPropagated, 3},
        {(int)Qt::WA_OpaquePaintEvent, 4},
        {(int)Qt::WA_NoBackground, 5},
        {(int)Qt::WA_StaticContents, 6},
        {(int)Qt::WA_LaidOut, 7},
        {(int)Qt::WA_PaintOnScreen, 8},
        {(int)Qt::WA_NoSystemBackground, 9},
        {(int)Qt::WA_UpdatesDisabled, 10},
        {(int)Qt::WA_Mapped, 11},
        {(int)Qt::WA_MacNoClickThrough, 12},
        {(int)Qt::WA_InputMethodEnabled, 13},
        {(int)Qt::WA_WState_Visible, 14},
        {(int)Qt::WA_WState_Hidden, 15},
        {(int)Qt::WA_ForceDisabled, 16},
        {(int)Qt::WA_KeyCompression, 17},
        {(int)Qt::WA_PendingMoveEvent, 18},
        {(int)Qt::WA_PendingResizeEvent, 19},
        {(int)Qt::WA_SetPalette, 20},
        {(int)Qt::WA_SetFont, 21},
        {(int)Qt::WA_SetCursor, 22},
        {(int)Qt::WA_NoChildEventsFromChildren, 23},
        {(int)Qt::WA_WindowModified, 24},
        {(int)Qt::WA_Resized, 25},
        {(int)Qt::WA_Moved, 26},
        {(int)Qt::WA_PendingUpdate, 27},
        {(int)Qt::WA_InvalidSize, 28},
        {(int)Qt::WA_MacBrushedMetal, 29},
        {(int)Qt::WA_MacMetalStyle, 30},
        {(int)Qt::WA_CustomWhatsThis, 31},
        {(int)Qt::WA_LayoutOnEntireRect, 32},
        {(int)Qt::WA_OutsideWSRange, 33},
        {(int)Qt::WA_GrabbedShortcut, 34},
        {(int)Qt::WA_TransparentForMouseEvents, 35},
        {(int)Qt::WA_PaintUnclipped, 36},
        {(int)Qt::WA_SetWindowIcon, 37},
        {(int)Qt::WA_NoMouseReplay, 38},
        {(int)Qt::WA_DeleteOnClose, 39},
        {(int)Qt::WA_RightToLeft, 40},
        {(int)Qt::WA_SetLayoutDirection, 41},
        {(int)Qt::WA_NoChildEventsForParent, 42},
        {(int)Qt::WA_ForceUpdatesDisabled, 43},
        {(int)Qt::WA_WState_Created, 44},
        {(int)Qt::WA_WState_CompressKeys, 45},
        {(int)Qt::WA_WState_InPaintEvent, 46},
        {(int)Qt::WA_WState_Reparented, 47},
        {(int)Qt::WA_WState_ConfigPending, 48},
        {(int)Qt::WA_WState_Polished, 49},
        {(int)Qt::WA_WState_DND, 50},
        {(int)Qt::WA_WState_OwnSizePolicy, 51},
        {(int)Qt::WA_WState_ExplicitShowHide, 52},
        {(int)Qt::WA_ShowModal, 53},
        {(int)Qt::WA_MouseNoMask, 54},
        {(int)Qt::WA_GroupLeader, 55},
        {(int)Qt::WA_NoMousePropagation, 56},
        {(int)Qt::WA_Hover, 57},
        {(int)Qt::WA_InputMethodTransparent, 58},
        {(int)Qt::WA_QuitOnClose, 59},
        {(int)Qt::WA_KeyboardFocusChange, 60},
        {(int)Qt::WA_AcceptDrops, 61},
        {(int)Qt::WA_DropSiteRegistered, 62},
        {(int)Qt::WA_ForceAcceptDrops, 63},
        {(int)Qt::WA_WindowPropagation, 64},
        {(int)Qt::WA_NoX11EventCompression, 65},
        {(int)Qt::WA_TintedBackground, 66},
        {(int)Qt::WA_X11OpenGLOverlay, 67},
        {(int)Qt::WA_AlwaysShowToolTips, 68},
        {(int)Qt::WA_MacOpaqueSizeGrip, 69},
        {(int)Qt::WA_SetStyle, 70},
        {(int)Qt::WA_SetLocale, 71},
        {(int)Qt::WA_MacShowFocusRect, 72},
        {(int)Qt::WA_MacNormalSize, 73},
        {(int)Qt::WA_MacSmallSize, 74},
        {(int)Qt::WA_MacMiniSize, 75},
        {(int)Qt::WA_LayoutUsesWidgetRect, 76},
        {(int)Qt::WA_StyledBackground, 77},
        {(int)Qt::WA_MSWindowsUseDirect3D, 78},
        {(int)Qt::WA_CanHostQMdiSubWindowTitleBar, 79},
        {(int)Qt::WA_MacAlwaysShowToolWindow, 80},
        {(int)Qt::WA_StyleSheet, 81},
        {(int)Qt::WA_ShowWithoutActivating, 82},
        {(int)Qt::WA_X11BypassTransientForHint, 83},
        {(int)Qt::WA_NativeWindow, 84},
        {(int)Qt::WA_DontCreateNativeAncestors, 85},
        {(int)Qt::WA_MacVariableSize, 86},
        {(int)Qt::WA_DontShowOnScreen, 87},
        {(int)Qt::WA_X11NetWmWindowTypeDesktop, 88},
        {(int)Qt::WA_X11NetWmWindowTypeDock, 89},
        {(int)Qt::WA_X11NetWmWindowTypeToolBar, 90},
        {(int)Qt::WA_X11NetWmWindowTypeMenu, 91},
        {(int)Qt::WA_X11NetWmWindowTypeUtility, 92},
        {(int)Qt::WA_X11NetWmWindowTypeSplash, 93},
        {(int)Qt::WA_X11NetWmWindowTypeDialog, 94},
        {(int)Qt::WA_X11NetWmWindowTypeDropDownMenu, 95},
        {(int)Qt::WA_X11NetWmWindowTypePopupMenu, 96},
        {(int)Qt::WA_X11NetWmWindowTypeToolTip, 97},
        {(int)Qt::WA_X11NetWmWindowTypeNotification, 98},
        {(int)Qt::WA_X11NetWmWindowTypeCombo, 99},
        {(int)Qt::WA_X11NetWmWindowTypeDND, 100},
        {(int)Qt::WA_MacFrameworkScaled, 101},
        {(int)Qt::WA_SetWindowModality, 102},
        {(int)Qt::WA_WState_WindowOpacitySet, 103},
        {(int)Qt::WA_TranslucentBackground, 104},
        {(int)Qt::WA_AcceptTouchEvents, 105},
        {(int)Qt::WA_WState_AcceptedTouchBeginEvent, 106},
        {(int)Qt::WA_TouchPadAcceptSingleTouchEvents, 107},
        {(int)Qt::WA_X11DoNotAcceptFocus, 108},
        {(int)Qt::WA_MacNoShadow, 109},
        {(int)Qt::WA_AlwaysStackOnTop, 110},
        {(int)Qt::WA_TabletTracking, 111},
        {(int)Qt::WA_ContentsMarginsRespectsSafeArea, 112},
        {(int)Qt::WA_AttributeCount, 113},
    };

    for (int i = 0; i < 114; ++i) {
        s_widget_attribute_lookup[widget_attribute_vals[i].key] =
            widget_attribute_vals[i].val;
    }

    static KeyVal window_frame_section_vals[] = {
        {(int)Qt::NoSection, 0},          {(int)Qt::LeftSection, 1},
        {(int)Qt::TopLeftSection, 2},     {(int)Qt::TopSection, 3},
        {(int)Qt::TopRightSection, 4},    {(int)Qt::RightSection, 5},
        {(int)Qt::BottomRightSection, 6}, {(int)Qt::BottomSection, 7},
        {(int)Qt::BottomLeftSection, 8},  {(int)Qt::TitleBarArea, 9},
    };

    for (int i = 0; i < 10; ++i) {
        s_window_frame_section_lookup[window_frame_section_vals[i].key] =
            window_frame_section_vals[i].val;
    }

    static KeyVal window_modality_vals[] = {
        {(int)Qt::NonModal, 0},
        {(int)Qt::WindowModal, 1},
        {(int)Qt::ApplicationModal, 2},
    };

    for (int i = 0; i < 3; ++i) {
        s_window_modality_lookup[window_modality_vals[i].key] =
            window_modality_vals[i].val;
    }

    static KeyVal window_state_vals[] = {
        {(int)Qt::WindowNoState, 0},   {(int)Qt::WindowMinimized, 1},
        {(int)Qt::WindowMaximized, 2}, {(int)Qt::WindowFullScreen, 3},
        {(int)Qt::WindowActive, 4},
    };

    for (int i = 0; i < 5; ++i) {
        s_window_state_lookup[window_state_vals[i].key] =
            window_state_vals[i].val;
    }

    static KeyVal window_type_vals[] = {
        {(int)Qt::Widget, 0},
        {(int)Qt::Window, 1},
        {(int)Qt::Dialog, 2},
        {(int)Qt::Sheet, 3},
        {(int)Qt::Drawer, 4},
        {(int)Qt::Popup, 5},
        {(int)Qt::Tool, 6},
        {(int)Qt::ToolTip, 7},
        {(int)Qt::SplashScreen, 8},
        {(int)Qt::Desktop, 9},
        {(int)Qt::SubWindow, 10},
        {(int)Qt::ForeignWindow, 11},
        {(int)Qt::CoverWindow, 12},
        {(int)Qt::WindowType_Mask, 13},
        {(int)Qt::MSWindowsFixedSizeDialogHint, 14},
        {(int)Qt::MSWindowsOwnDC, 15},
        {(int)Qt::BypassWindowManagerHint, 16},
        {(int)Qt::X11BypassWindowManagerHint, 17},
        {(int)Qt::FramelessWindowHint, 18},
        {(int)Qt::WindowTitleHint, 19},
        {(int)Qt::WindowSystemMenuHint, 20},
        {(int)Qt::WindowMinimizeButtonHint, 21},
        {(int)Qt::WindowMaximizeButtonHint, 22},
        {(int)Qt::WindowMinMaxButtonsHint, 23},
        {(int)Qt::WindowContextHelpButtonHint, 24},
        {(int)Qt::WindowShadeButtonHint, 25},
        {(int)Qt::WindowStaysOnTopHint, 26},
        {(int)Qt::WindowTransparentForInput, 27},
        {(int)Qt::WindowOverridesSystemGestures, 28},
        {(int)Qt::WindowDoesNotAcceptFocus, 29},
        {(int)Qt::MaximizeUsingFullscreenGeometryHint, 30},
        {(int)Qt::CustomizeWindowHint, 31},
        {(int)Qt::WindowStaysOnBottomHint, 32},
        {(int)Qt::WindowCloseButtonHint, 33},
        {(int)Qt::MacWindowToolBarButtonHint, 34},
        {(int)Qt::BypassGraphicsProxyWidget, 35},
        {(int)Qt::NoDropShadowWindowHint, 36},
        {(int)Qt::WindowFullscreenButtonHint, 37},
    };

    for (int i = 0; i < 38; ++i) {
        s_window_type_lookup[window_type_vals[i].key] = window_type_vals[i].val;
    }

    static KeyVal yuv_layout_vals[] = {
        {(int)QPixelFormat::YUV444, 0},   {(int)QPixelFormat::YUV422, 1},
        {(int)QPixelFormat::YUV411, 2},   {(int)QPixelFormat::YUV420P, 3},
        {(int)QPixelFormat::YUV420SP, 4}, {(int)QPixelFormat::YV12, 5},
        {(int)QPixelFormat::UYVY, 6},     {(int)QPixelFormat::YUYV, 7},
        {(int)QPixelFormat::NV12, 8},     {(int)QPixelFormat::NV21, 9},
        {(int)QPixelFormat::IMC1, 10},    {(int)QPixelFormat::IMC2, 11},
        {(int)QPixelFormat::IMC3, 12},    {(int)QPixelFormat::IMC4, 13},
        {(int)QPixelFormat::Y8, 14},      {(int)QPixelFormat::Y16, 15},
    };

    for (int i = 0; i < 16; ++i) {
        s_yuv_layout_lookup[yuv_layout_vals[i].key] = yuv_layout_vals[i].val;
    }
}
