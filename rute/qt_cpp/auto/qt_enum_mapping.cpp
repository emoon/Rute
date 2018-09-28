#include <QListWidgetItem>
#include <Qt>
#include <map>

struct KeyVal { int val, key; };

std::map<int, int> s_alignment_flag_lookup;
std::map<int, int> s_anchor_point_lookup;
std::map<int, int> s_application_attribute_lookup;
std::map<int, int> s_application_state_lookup;
std::map<int, int> s_arrow_type_lookup;
std::map<int, int> s_aspect_ratio_mode_lookup;
std::map<int, int> s_axis_lookup;
std::map<int, int> s_bg_mode_lookup;
std::map<int, int> s_brush_style_lookup;
std::map<int, int> s_case_sensitivity_lookup;
std::map<int, int> s_check_state_lookup;
std::map<int, int> s_checksum_type_lookup;
std::map<int, int> s_clip_operation_lookup;
std::map<int, int> s_connection_type_lookup;
std::map<int, int> s_context_menu_policy_lookup;
std::map<int, int> s_coordinate_system_lookup;
std::map<int, int> s_corner_lookup;
std::map<int, int> s_cursor_move_style_lookup;
std::map<int, int> s_cursor_shape_lookup;
std::map<int, int> s_date_format_lookup;
std::map<int, int> s_day_of_week_lookup;
std::map<int, int> s_dock_widget_area_lookup;
std::map<int, int> s_dock_widget_area_sizes_lookup;
std::map<int, int> s_drop_action_lookup;
std::map<int, int> s_edge_lookup;
std::map<int, int> s_enter_key_type_lookup;
std::map<int, int> s_event_priority_lookup;
std::map<int, int> s_fill_rule_lookup;
std::map<int, int> s_find_child_option_lookup;
std::map<int, int> s_focus_policy_lookup;
std::map<int, int> s_focus_reason_lookup;
std::map<int, int> s_gesture_flag_lookup;
std::map<int, int> s_gesture_state_lookup;
std::map<int, int> s_gesture_type_lookup;
std::map<int, int> s_global_color_lookup;
std::map<int, int> s_hit_test_accuracy_lookup;
std::map<int, int> s_image_conversion_flag_lookup;
std::map<int, int> s_initialization_lookup;
std::map<int, int> s_input_method_hint_lookup;
std::map<int, int> s_input_method_query_lookup;
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
std::map<int, int> s_modifier_lookup;
std::map<int, int> s_mouse_button_lookup;
std::map<int, int> s_mouse_event_flag_lookup;
std::map<int, int> s_mouse_event_source_lookup;
std::map<int, int> s_native_gesture_type_lookup;
std::map<int, int> s_navigation_mode_lookup;
std::map<int, int> s_orientation_lookup;
std::map<int, int> s_pen_cap_style_lookup;
std::map<int, int> s_pen_join_style_lookup;
std::map<int, int> s_pen_style_lookup;
std::map<int, int> s_screen_orientation_lookup;
std::map<int, int> s_scroll_bar_policy_lookup;
std::map<int, int> s_scroll_phase_lookup;
std::map<int, int> s_shortcut_context_lookup;
std::map<int, int> s_size_hint_lookup;
std::map<int, int> s_size_mode_lookup;
std::map<int, int> s_sort_order_lookup;
std::map<int, int> s_tab_focus_behavior_lookup;
std::map<int, int> s_text_elide_mode_lookup;
std::map<int, int> s_text_flag_lookup;
std::map<int, int> s_text_format_lookup;
std::map<int, int> s_text_interaction_flag_lookup;
std::map<int, int> s_tile_rule_lookup;
std::map<int, int> s_time_spec_lookup;
std::map<int, int> s_timer_type_lookup;
std::map<int, int> s_tool_bar_area_lookup;
std::map<int, int> s_tool_bar_area_sizes_lookup;
std::map<int, int> s_tool_button_style_lookup;
std::map<int, int> s_touch_point_state_lookup;
std::map<int, int> s_transformation_mode_lookup;
std::map<int, int> s_ui_effect_lookup;
std::map<int, int> s_white_space_mode_lookup;
std::map<int, int> s_widget_attribute_lookup;
std::map<int, int> s_window_frame_section_lookup;
std::map<int, int> s_window_modality_lookup;
std::map<int, int> s_window_state_lookup;
std::map<int, int> s_window_type_lookup;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern void create_enum_mappings() {

    static KeyVal alignment_flag_vals[] =
    {{  (int)Qt::AlignLeft, 0 },
    {  (int)Qt::AlignLeading, 1 },
    {  (int)Qt::AlignRight, 2 },
    {  (int)Qt::AlignTrailing, 3 },
    {  (int)Qt::AlignHCenter, 4 },
    {  (int)Qt::AlignJustify, 5 },
    {  (int)Qt::AlignAbsolute, 6 },
    {  (int)Qt::AlignHorizontal_Mask, 7 },
    {  (int)Qt::AlignTop, 8 },
    {  (int)Qt::AlignBottom, 9 },
    {  (int)Qt::AlignVCenter, 10 },
    {  (int)Qt::AlignBaseline, 11 },
    {  (int)Qt::AlignVertical_Mask, 12 },
    {  (int)Qt::AlignCenter, 13 },
    };

    for (int i = 0; i < 14; ++i) {
        s_alignment_flag_lookup[alignment_flag_vals[i].key] = alignment_flag_vals[i].val;
    }

    static KeyVal anchor_point_vals[] =
    {{  (int)Qt::AnchorLeft, 0 },
    {  (int)Qt::AnchorHorizontalCenter, 1 },
    {  (int)Qt::AnchorRight, 2 },
    {  (int)Qt::AnchorTop, 3 },
    {  (int)Qt::AnchorVerticalCenter, 4 },
    {  (int)Qt::AnchorBottom, 5 },
    };

    for (int i = 0; i < 6; ++i) {
        s_anchor_point_lookup[anchor_point_vals[i].key] = anchor_point_vals[i].val;
    }

    static KeyVal application_attribute_vals[] =
    {{  (int)Qt::AA_ImmediateWidgetCreation, 0 },
    {  (int)Qt::AA_MSWindowsUseDirect3DByDefault, 1 },
    {  (int)Qt::AA_DontShowIconsInMenus, 2 },
    {  (int)Qt::AA_NativeWindows, 3 },
    {  (int)Qt::AA_DontCreateNativeWidgetSiblings, 4 },
    {  (int)Qt::AA_PluginApplication, 5 },
    {  (int)Qt::AA_MacPluginApplication, 6 },
    {  (int)Qt::AA_DontUseNativeMenuBar, 7 },
    {  (int)Qt::AA_MacDontSwapCtrlAndMeta, 8 },
    {  (int)Qt::AA_Use96Dpi, 9 },
    {  (int)Qt::AA_X11InitThreads, 10 },
    {  (int)Qt::AA_SynthesizeTouchForUnhandledMouseEvents, 11 },
    {  (int)Qt::AA_SynthesizeMouseForUnhandledTouchEvents, 12 },
    {  (int)Qt::AA_UseHighDpiPixmaps, 13 },
    {  (int)Qt::AA_ForceRasterWidgets, 14 },
    {  (int)Qt::AA_UseDesktopOpenGL, 15 },
    {  (int)Qt::AA_UseOpenGLES, 16 },
    {  (int)Qt::AA_UseSoftwareOpenGL, 17 },
    {  (int)Qt::AA_ShareOpenGLContexts, 18 },
    {  (int)Qt::AA_SetPalette, 19 },
    {  (int)Qt::AA_EnableHighDpiScaling, 20 },
    {  (int)Qt::AA_DisableHighDpiScaling, 21 },
    {  (int)Qt::AA_UseStyleSheetPropagationInWidgetStyles, 22 },
    {  (int)Qt::AA_DontUseNativeDialogs, 23 },
    {  (int)Qt::AA_SynthesizeMouseForUnhandledTabletEvents, 24 },
    {  (int)Qt::AA_CompressHighFrequencyEvents, 25 },
    {  (int)Qt::AA_DontCheckOpenGLContextThreadAffinity, 26 },
    {  (int)Qt::AA_DisableShaderDiskCache, 27 },
    {  (int)Qt::AA_DontShowShortcutsInContextMenus, 28 },
    {  (int)Qt::AA_CompressTabletEvents, 29 },
    {  (int)Qt::AA_DisableWindowContextHelpButton, 30 },
    {  (int)Qt::AA_AttributeCount, 31 },
    };

    for (int i = 0; i < 32; ++i) {
        s_application_attribute_lookup[application_attribute_vals[i].key] = application_attribute_vals[i].val;
    }

    static KeyVal application_state_vals[] =
    {{  (int)Qt::ApplicationSuspended, 0 },
    {  (int)Qt::ApplicationHidden, 1 },
    {  (int)Qt::ApplicationInactive, 2 },
    {  (int)Qt::ApplicationActive, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_application_state_lookup[application_state_vals[i].key] = application_state_vals[i].val;
    }

    static KeyVal arrow_type_vals[] =
    {{  (int)Qt::NoArrow, 0 },
    {  (int)Qt::UpArrow, 1 },
    {  (int)Qt::DownArrow, 2 },
    {  (int)Qt::LeftArrow, 3 },
    {  (int)Qt::RightArrow, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_arrow_type_lookup[arrow_type_vals[i].key] = arrow_type_vals[i].val;
    }

    static KeyVal aspect_ratio_mode_vals[] =
    {{  (int)Qt::IgnoreAspectRatio, 0 },
    {  (int)Qt::KeepAspectRatio, 1 },
    {  (int)Qt::KeepAspectRatioByExpanding, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_aspect_ratio_mode_lookup[aspect_ratio_mode_vals[i].key] = aspect_ratio_mode_vals[i].val;
    }

    static KeyVal axis_vals[] =
    {{  (int)Qt::XAxis, 0 },
    {  (int)Qt::YAxis, 1 },
    {  (int)Qt::ZAxis, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_axis_lookup[axis_vals[i].key] = axis_vals[i].val;
    }

    static KeyVal bg_mode_vals[] =
    {{  (int)Qt::TransparentMode, 0 },
    {  (int)Qt::OpaqueMode, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_bg_mode_lookup[bg_mode_vals[i].key] = bg_mode_vals[i].val;
    }

    static KeyVal brush_style_vals[] =
    {{  (int)Qt::NoBrush, 0 },
    {  (int)Qt::SolidPattern, 1 },
    {  (int)Qt::Dense1Pattern, 2 },
    {  (int)Qt::Dense2Pattern, 3 },
    {  (int)Qt::Dense3Pattern, 4 },
    {  (int)Qt::Dense4Pattern, 5 },
    {  (int)Qt::Dense5Pattern, 6 },
    {  (int)Qt::Dense6Pattern, 7 },
    {  (int)Qt::Dense7Pattern, 8 },
    {  (int)Qt::HorPattern, 9 },
    {  (int)Qt::VerPattern, 10 },
    {  (int)Qt::CrossPattern, 11 },
    {  (int)Qt::BDiagPattern, 12 },
    {  (int)Qt::FDiagPattern, 13 },
    {  (int)Qt::DiagCrossPattern, 14 },
    {  (int)Qt::LinearGradientPattern, 15 },
    {  (int)Qt::RadialGradientPattern, 16 },
    {  (int)Qt::ConicalGradientPattern, 17 },
    {  (int)Qt::TexturePattern, 18 },
    };

    for (int i = 0; i < 19; ++i) {
        s_brush_style_lookup[brush_style_vals[i].key] = brush_style_vals[i].val;
    }

    static KeyVal case_sensitivity_vals[] =
    {{  (int)Qt::CaseInsensitive, 0 },
    {  (int)Qt::CaseSensitive, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_case_sensitivity_lookup[case_sensitivity_vals[i].key] = case_sensitivity_vals[i].val;
    }

    static KeyVal check_state_vals[] =
    {{  (int)Qt::Unchecked, 0 },
    {  (int)Qt::PartiallyChecked, 1 },
    {  (int)Qt::Checked, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_check_state_lookup[check_state_vals[i].key] = check_state_vals[i].val;
    }

    static KeyVal checksum_type_vals[] =
    {{  (int)Qt::ChecksumIso3309, 0 },
    {  (int)Qt::ChecksumItuV41, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_checksum_type_lookup[checksum_type_vals[i].key] = checksum_type_vals[i].val;
    }

    static KeyVal clip_operation_vals[] =
    {{  (int)Qt::NoClip, 0 },
    {  (int)Qt::ReplaceClip, 1 },
    {  (int)Qt::IntersectClip, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_clip_operation_lookup[clip_operation_vals[i].key] = clip_operation_vals[i].val;
    }

    static KeyVal connection_type_vals[] =
    {{  (int)Qt::AutoConnection, 0 },
    {  (int)Qt::DirectConnection, 1 },
    {  (int)Qt::QueuedConnection, 2 },
    {  (int)Qt::BlockingQueuedConnection, 3 },
    {  (int)Qt::UniqueConnection, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_connection_type_lookup[connection_type_vals[i].key] = connection_type_vals[i].val;
    }

    static KeyVal context_menu_policy_vals[] =
    {{  (int)Qt::NoContextMenu, 0 },
    {  (int)Qt::DefaultContextMenu, 1 },
    {  (int)Qt::ActionsContextMenu, 2 },
    {  (int)Qt::CustomContextMenu, 3 },
    {  (int)Qt::PreventContextMenu, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_context_menu_policy_lookup[context_menu_policy_vals[i].key] = context_menu_policy_vals[i].val;
    }

    static KeyVal coordinate_system_vals[] =
    {{  (int)Qt::DeviceCoordinates, 0 },
    {  (int)Qt::LogicalCoordinates, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_coordinate_system_lookup[coordinate_system_vals[i].key] = coordinate_system_vals[i].val;
    }

    static KeyVal corner_vals[] =
    {{  (int)Qt::TopLeftCorner, 0 },
    {  (int)Qt::TopRightCorner, 1 },
    {  (int)Qt::BottomLeftCorner, 2 },
    {  (int)Qt::BottomRightCorner, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_corner_lookup[corner_vals[i].key] = corner_vals[i].val;
    }

    static KeyVal cursor_move_style_vals[] =
    {{  (int)Qt::LogicalMoveStyle, 0 },
    {  (int)Qt::VisualMoveStyle, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_cursor_move_style_lookup[cursor_move_style_vals[i].key] = cursor_move_style_vals[i].val;
    }

    static KeyVal cursor_shape_vals[] =
    {{  (int)Qt::ArrowCursor, 0 },
    {  (int)Qt::UpArrowCursor, 1 },
    {  (int)Qt::CrossCursor, 2 },
    {  (int)Qt::WaitCursor, 3 },
    {  (int)Qt::IBeamCursor, 4 },
    {  (int)Qt::SizeVerCursor, 5 },
    {  (int)Qt::SizeHorCursor, 6 },
    {  (int)Qt::SizeBDiagCursor, 7 },
    {  (int)Qt::SizeFDiagCursor, 8 },
    {  (int)Qt::SizeAllCursor, 9 },
    {  (int)Qt::BlankCursor, 10 },
    {  (int)Qt::SplitVCursor, 11 },
    {  (int)Qt::SplitHCursor, 12 },
    {  (int)Qt::PointingHandCursor, 13 },
    {  (int)Qt::ForbiddenCursor, 14 },
    {  (int)Qt::WhatsThisCursor, 15 },
    {  (int)Qt::BusyCursor, 16 },
    {  (int)Qt::OpenHandCursor, 17 },
    {  (int)Qt::ClosedHandCursor, 18 },
    {  (int)Qt::DragCopyCursor, 19 },
    {  (int)Qt::DragMoveCursor, 20 },
    {  (int)Qt::DragLinkCursor, 21 },
    {  (int)Qt::LastCursor, 22 },
    {  (int)Qt::BitmapCursor, 23 },
    {  (int)Qt::CustomCursor, 24 },
    };

    for (int i = 0; i < 25; ++i) {
        s_cursor_shape_lookup[cursor_shape_vals[i].key] = cursor_shape_vals[i].val;
    }

    static KeyVal date_format_vals[] =
    {{  (int)Qt::TextDate, 0 },
    {  (int)Qt::ISODate, 1 },
    {  (int)Qt::SystemLocaleDate, 2 },
    {  (int)Qt::LocalDate, 3 },
    {  (int)Qt::LocaleDate, 4 },
    {  (int)Qt::SystemLocaleShortDate, 5 },
    {  (int)Qt::SystemLocaleLongDate, 6 },
    {  (int)Qt::DefaultLocaleShortDate, 7 },
    {  (int)Qt::DefaultLocaleLongDate, 8 },
    {  (int)Qt::RFC2822Date, 9 },
    {  (int)Qt::ISODateWithMs, 10 },
    };

    for (int i = 0; i < 11; ++i) {
        s_date_format_lookup[date_format_vals[i].key] = date_format_vals[i].val;
    }

    static KeyVal day_of_week_vals[] =
    {{  (int)Qt::Monday, 0 },
    {  (int)Qt::Tuesday, 1 },
    {  (int)Qt::Wednesday, 2 },
    {  (int)Qt::Thursday, 3 },
    {  (int)Qt::Friday, 4 },
    {  (int)Qt::Saturday, 5 },
    {  (int)Qt::Sunday, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_day_of_week_lookup[day_of_week_vals[i].key] = day_of_week_vals[i].val;
    }

    static KeyVal dock_widget_area_vals[] =
    {{  (int)Qt::LeftDockWidgetArea, 0 },
    {  (int)Qt::RightDockWidgetArea, 1 },
    {  (int)Qt::TopDockWidgetArea, 2 },
    {  (int)Qt::BottomDockWidgetArea, 3 },
    {  (int)Qt::DockWidgetArea_Mask, 4 },
    {  (int)Qt::AllDockWidgetAreas, 5 },
    {  (int)Qt::NoDockWidgetArea, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_dock_widget_area_lookup[dock_widget_area_vals[i].key] = dock_widget_area_vals[i].val;
    }

    static KeyVal dock_widget_area_sizes_vals[] =
    {{  (int)Qt::NDockWidgetAreas, 0 },
    };

    for (int i = 0; i < 1; ++i) {
        s_dock_widget_area_sizes_lookup[dock_widget_area_sizes_vals[i].key] = dock_widget_area_sizes_vals[i].val;
    }

    static KeyVal drop_action_vals[] =
    {{  (int)Qt::CopyAction, 0 },
    {  (int)Qt::MoveAction, 1 },
    {  (int)Qt::LinkAction, 2 },
    {  (int)Qt::ActionMask, 3 },
    {  (int)Qt::TargetMoveAction, 4 },
    {  (int)Qt::IgnoreAction, 5 },
    };

    for (int i = 0; i < 6; ++i) {
        s_drop_action_lookup[drop_action_vals[i].key] = drop_action_vals[i].val;
    }

    static KeyVal edge_vals[] =
    {{  (int)Qt::TopEdge, 0 },
    {  (int)Qt::LeftEdge, 1 },
    {  (int)Qt::RightEdge, 2 },
    {  (int)Qt::BottomEdge, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_edge_lookup[edge_vals[i].key] = edge_vals[i].val;
    }

    static KeyVal enter_key_type_vals[] =
    {{  (int)Qt::EnterKeyDefault, 0 },
    {  (int)Qt::EnterKeyReturn, 1 },
    {  (int)Qt::EnterKeyDone, 2 },
    {  (int)Qt::EnterKeyGo, 3 },
    {  (int)Qt::EnterKeySend, 4 },
    {  (int)Qt::EnterKeySearch, 5 },
    {  (int)Qt::EnterKeyNext, 6 },
    {  (int)Qt::EnterKeyPrevious, 7 },
    };

    for (int i = 0; i < 8; ++i) {
        s_enter_key_type_lookup[enter_key_type_vals[i].key] = enter_key_type_vals[i].val;
    }

    static KeyVal event_priority_vals[] =
    {{  (int)Qt::HighEventPriority, 0 },
    {  (int)Qt::NormalEventPriority, 1 },
    {  (int)Qt::LowEventPriority, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_event_priority_lookup[event_priority_vals[i].key] = event_priority_vals[i].val;
    }

    static KeyVal fill_rule_vals[] =
    {{  (int)Qt::OddEvenFill, 0 },
    {  (int)Qt::WindingFill, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_fill_rule_lookup[fill_rule_vals[i].key] = fill_rule_vals[i].val;
    }

    static KeyVal find_child_option_vals[] =
    {{  (int)Qt::FindDirectChildrenOnly, 0 },
    {  (int)Qt::FindChildrenRecursively, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_find_child_option_lookup[find_child_option_vals[i].key] = find_child_option_vals[i].val;
    }

    static KeyVal focus_policy_vals[] =
    {{  (int)Qt::NoFocus, 0 },
    {  (int)Qt::TabFocus, 1 },
    {  (int)Qt::ClickFocus, 2 },
    {  (int)Qt::StrongFocus, 3 },
    {  (int)Qt::WheelFocus, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_focus_policy_lookup[focus_policy_vals[i].key] = focus_policy_vals[i].val;
    }

    static KeyVal focus_reason_vals[] =
    {{  (int)Qt::MouseFocusReason, 0 },
    {  (int)Qt::TabFocusReason, 1 },
    {  (int)Qt::BacktabFocusReason, 2 },
    {  (int)Qt::ActiveWindowFocusReason, 3 },
    {  (int)Qt::PopupFocusReason, 4 },
    {  (int)Qt::ShortcutFocusReason, 5 },
    {  (int)Qt::MenuBarFocusReason, 6 },
    {  (int)Qt::OtherFocusReason, 7 },
    {  (int)Qt::NoFocusReason, 8 },
    };

    for (int i = 0; i < 9; ++i) {
        s_focus_reason_lookup[focus_reason_vals[i].key] = focus_reason_vals[i].val;
    }

    static KeyVal gesture_flag_vals[] =
    {{  (int)Qt::DontStartGestureOnChildren, 0 },
    {  (int)Qt::ReceivePartialGestures, 1 },
    {  (int)Qt::IgnoredGesturesPropagateToParent, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_gesture_flag_lookup[gesture_flag_vals[i].key] = gesture_flag_vals[i].val;
    }

    static KeyVal gesture_state_vals[] =
    {{  (int)Qt::NoGesture, 0 },
    {  (int)Qt::GestureStarted, 1 },
    {  (int)Qt::GestureUpdated, 2 },
    {  (int)Qt::GestureFinished, 3 },
    {  (int)Qt::GestureCanceled, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_gesture_state_lookup[gesture_state_vals[i].key] = gesture_state_vals[i].val;
    }

    static KeyVal gesture_type_vals[] =
    {{  (int)Qt::TapGesture, 0 },
    {  (int)Qt::TapAndHoldGesture, 1 },
    {  (int)Qt::PanGesture, 2 },
    {  (int)Qt::PinchGesture, 3 },
    {  (int)Qt::SwipeGesture, 4 },
    {  (int)Qt::CustomGesture, 5 },
    {  (int)Qt::LastGestureType, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_gesture_type_lookup[gesture_type_vals[i].key] = gesture_type_vals[i].val;
    }

    static KeyVal global_color_vals[] =
    {{  (int)Qt::color0, 0 },
    {  (int)Qt::color1, 1 },
    {  (int)Qt::black, 2 },
    {  (int)Qt::white, 3 },
    {  (int)Qt::darkGray, 4 },
    {  (int)Qt::gray, 5 },
    {  (int)Qt::lightGray, 6 },
    {  (int)Qt::red, 7 },
    {  (int)Qt::green, 8 },
    {  (int)Qt::blue, 9 },
    {  (int)Qt::cyan, 10 },
    {  (int)Qt::magenta, 11 },
    {  (int)Qt::yellow, 12 },
    {  (int)Qt::darkRed, 13 },
    {  (int)Qt::darkGreen, 14 },
    {  (int)Qt::darkBlue, 15 },
    {  (int)Qt::darkCyan, 16 },
    {  (int)Qt::darkMagenta, 17 },
    {  (int)Qt::darkYellow, 18 },
    {  (int)Qt::transparent, 19 },
    };

    for (int i = 0; i < 20; ++i) {
        s_global_color_lookup[global_color_vals[i].key] = global_color_vals[i].val;
    }

    static KeyVal hit_test_accuracy_vals[] =
    {{  (int)Qt::ExactHit, 0 },
    {  (int)Qt::FuzzyHit, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_hit_test_accuracy_lookup[hit_test_accuracy_vals[i].key] = hit_test_accuracy_vals[i].val;
    }

    static KeyVal image_conversion_flag_vals[] =
    {{  (int)Qt::ColorMode_Mask, 0 },
    {  (int)Qt::AutoColor, 1 },
    {  (int)Qt::ColorOnly, 2 },
    {  (int)Qt::MonoOnly, 3 },
    {  (int)Qt::AlphaDither_Mask, 4 },
    {  (int)Qt::ThresholdAlphaDither, 5 },
    {  (int)Qt::OrderedAlphaDither, 6 },
    {  (int)Qt::DiffuseAlphaDither, 7 },
    {  (int)Qt::NoAlpha, 8 },
    {  (int)Qt::Dither_Mask, 9 },
    {  (int)Qt::DiffuseDither, 10 },
    {  (int)Qt::OrderedDither, 11 },
    {  (int)Qt::ThresholdDither, 12 },
    {  (int)Qt::DitherMode_Mask, 13 },
    {  (int)Qt::AutoDither, 14 },
    {  (int)Qt::PreferDither, 15 },
    {  (int)Qt::AvoidDither, 16 },
    {  (int)Qt::NoOpaqueDetection, 17 },
    {  (int)Qt::NoFormatConversion, 18 },
    };

    for (int i = 0; i < 19; ++i) {
        s_image_conversion_flag_lookup[image_conversion_flag_vals[i].key] = image_conversion_flag_vals[i].val;
    }

    static KeyVal initialization_vals[] =
    {{  (int)Qt::Uninitialized, 0 },
    };

    for (int i = 0; i < 1; ++i) {
        s_initialization_lookup[initialization_vals[i].key] = initialization_vals[i].val;
    }

    static KeyVal input_method_hint_vals[] =
    {{  (int)Qt::ImhNone, 0 },
    {  (int)Qt::ImhHiddenText, 1 },
    {  (int)Qt::ImhSensitiveData, 2 },
    {  (int)Qt::ImhNoAutoUppercase, 3 },
    {  (int)Qt::ImhPreferNumbers, 4 },
    {  (int)Qt::ImhPreferUppercase, 5 },
    {  (int)Qt::ImhPreferLowercase, 6 },
    {  (int)Qt::ImhNoPredictiveText, 7 },
    {  (int)Qt::ImhDate, 8 },
    {  (int)Qt::ImhTime, 9 },
    {  (int)Qt::ImhPreferLatin, 10 },
    {  (int)Qt::ImhMultiLine, 11 },
    {  (int)Qt::ImhDigitsOnly, 12 },
    {  (int)Qt::ImhFormattedNumbersOnly, 13 },
    {  (int)Qt::ImhUppercaseOnly, 14 },
    {  (int)Qt::ImhLowercaseOnly, 15 },
    {  (int)Qt::ImhDialableCharactersOnly, 16 },
    {  (int)Qt::ImhEmailCharactersOnly, 17 },
    {  (int)Qt::ImhUrlCharactersOnly, 18 },
    {  (int)Qt::ImhLatinOnly, 19 },
    {  (int)Qt::ImhExclusiveInputMask, 20 },
    };

    for (int i = 0; i < 21; ++i) {
        s_input_method_hint_lookup[input_method_hint_vals[i].key] = input_method_hint_vals[i].val;
    }

    static KeyVal input_method_query_vals[] =
    {{  (int)Qt::ImEnabled, 0 },
    {  (int)Qt::ImCursorRectangle, 1 },
    {  (int)Qt::ImMicroFocus, 2 },
    {  (int)Qt::ImFont, 3 },
    {  (int)Qt::ImCursorPosition, 4 },
    {  (int)Qt::ImSurroundingText, 5 },
    {  (int)Qt::ImCurrentSelection, 6 },
    {  (int)Qt::ImMaximumTextLength, 7 },
    {  (int)Qt::ImAnchorPosition, 8 },
    {  (int)Qt::ImHints, 9 },
    {  (int)Qt::ImPreferredLanguage, 10 },
    {  (int)Qt::ImAbsolutePosition, 11 },
    {  (int)Qt::ImTextBeforeCursor, 12 },
    {  (int)Qt::ImTextAfterCursor, 13 },
    {  (int)Qt::ImEnterKeyType, 14 },
    {  (int)Qt::ImAnchorRectangle, 15 },
    {  (int)Qt::ImInputItemClipRectangle, 16 },
    {  (int)Qt::ImPlatformData, 17 },
    {  (int)Qt::ImQueryInput, 18 },
    {  (int)Qt::ImQueryAll, 19 },
    };

    for (int i = 0; i < 20; ++i) {
        s_input_method_query_lookup[input_method_query_vals[i].key] = input_method_query_vals[i].val;
    }

    static KeyVal item_data_role_vals[] =
    {{  (int)Qt::DisplayRole, 0 },
    {  (int)Qt::DecorationRole, 1 },
    {  (int)Qt::EditRole, 2 },
    {  (int)Qt::ToolTipRole, 3 },
    {  (int)Qt::StatusTipRole, 4 },
    {  (int)Qt::WhatsThisRole, 5 },
    {  (int)Qt::FontRole, 6 },
    {  (int)Qt::TextAlignmentRole, 7 },
    {  (int)Qt::BackgroundColorRole, 8 },
    {  (int)Qt::BackgroundRole, 9 },
    {  (int)Qt::TextColorRole, 10 },
    {  (int)Qt::ForegroundRole, 11 },
    {  (int)Qt::CheckStateRole, 12 },
    {  (int)Qt::AccessibleTextRole, 13 },
    {  (int)Qt::AccessibleDescriptionRole, 14 },
    {  (int)Qt::SizeHintRole, 15 },
    {  (int)Qt::InitialSortOrderRole, 16 },
    {  (int)Qt::DisplayPropertyRole, 17 },
    {  (int)Qt::DecorationPropertyRole, 18 },
    {  (int)Qt::ToolTipPropertyRole, 19 },
    {  (int)Qt::StatusTipPropertyRole, 20 },
    {  (int)Qt::WhatsThisPropertyRole, 21 },
    {  (int)Qt::UserRole, 22 },
    };

    for (int i = 0; i < 23; ++i) {
        s_item_data_role_lookup[item_data_role_vals[i].key] = item_data_role_vals[i].val;
    }

    static KeyVal item_flag_vals[] =
    {{  (int)Qt::NoItemFlags, 0 },
    {  (int)Qt::ItemIsSelectable, 1 },
    {  (int)Qt::ItemIsEditable, 2 },
    {  (int)Qt::ItemIsDragEnabled, 3 },
    {  (int)Qt::ItemIsDropEnabled, 4 },
    {  (int)Qt::ItemIsUserCheckable, 5 },
    {  (int)Qt::ItemIsEnabled, 6 },
    {  (int)Qt::ItemIsAutoTristate, 7 },
    {  (int)Qt::ItemIsTristate, 8 },
    {  (int)Qt::ItemNeverHasChildren, 9 },
    {  (int)Qt::ItemIsUserTristate, 10 },
    };

    for (int i = 0; i < 11; ++i) {
        s_item_flag_lookup[item_flag_vals[i].key] = item_flag_vals[i].val;
    }

    static KeyVal item_selection_mode_vals[] =
    {{  (int)Qt::ContainsItemShape, 0 },
    {  (int)Qt::IntersectsItemShape, 1 },
    {  (int)Qt::ContainsItemBoundingRect, 2 },
    {  (int)Qt::IntersectsItemBoundingRect, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_item_selection_mode_lookup[item_selection_mode_vals[i].key] = item_selection_mode_vals[i].val;
    }

    static KeyVal item_selection_operation_vals[] =
    {{  (int)Qt::ReplaceSelection, 0 },
    {  (int)Qt::AddToSelection, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_item_selection_operation_lookup[item_selection_operation_vals[i].key] = item_selection_operation_vals[i].val;
    }

    static KeyVal item_type_vals[] =
    {{  (int)QListWidgetItem::Type, 0 },
    {  (int)QListWidgetItem::UserType, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_item_type_lookup[item_type_vals[i].key] = item_type_vals[i].val;
    }

    static KeyVal key_vals[] =
    {{  (int)Qt::Key_Escape, 0 },
    {  (int)Qt::Key_Tab, 1 },
    {  (int)Qt::Key_Backtab, 2 },
    {  (int)Qt::Key_Backspace, 3 },
    {  (int)Qt::Key_Return, 4 },
    {  (int)Qt::Key_Enter, 5 },
    {  (int)Qt::Key_Insert, 6 },
    {  (int)Qt::Key_Delete, 7 },
    {  (int)Qt::Key_Pause, 8 },
    {  (int)Qt::Key_Print, 9 },
    {  (int)Qt::Key_SysReq, 10 },
    {  (int)Qt::Key_Clear, 11 },
    {  (int)Qt::Key_Home, 12 },
    {  (int)Qt::Key_End, 13 },
    {  (int)Qt::Key_Left, 14 },
    {  (int)Qt::Key_Up, 15 },
    {  (int)Qt::Key_Right, 16 },
    {  (int)Qt::Key_Down, 17 },
    {  (int)Qt::Key_PageUp, 18 },
    {  (int)Qt::Key_PageDown, 19 },
    {  (int)Qt::Key_Shift, 20 },
    {  (int)Qt::Key_Control, 21 },
    {  (int)Qt::Key_Meta, 22 },
    {  (int)Qt::Key_Alt, 23 },
    {  (int)Qt::Key_CapsLock, 24 },
    {  (int)Qt::Key_NumLock, 25 },
    {  (int)Qt::Key_ScrollLock, 26 },
    {  (int)Qt::Key_F1, 27 },
    {  (int)Qt::Key_F2, 28 },
    {  (int)Qt::Key_F3, 29 },
    {  (int)Qt::Key_F4, 30 },
    {  (int)Qt::Key_F5, 31 },
    {  (int)Qt::Key_F6, 32 },
    {  (int)Qt::Key_F7, 33 },
    {  (int)Qt::Key_F8, 34 },
    {  (int)Qt::Key_F9, 35 },
    {  (int)Qt::Key_F10, 36 },
    {  (int)Qt::Key_F11, 37 },
    {  (int)Qt::Key_F12, 38 },
    {  (int)Qt::Key_F13, 39 },
    {  (int)Qt::Key_F14, 40 },
    {  (int)Qt::Key_F15, 41 },
    {  (int)Qt::Key_F16, 42 },
    {  (int)Qt::Key_F17, 43 },
    {  (int)Qt::Key_F18, 44 },
    {  (int)Qt::Key_F19, 45 },
    {  (int)Qt::Key_F20, 46 },
    {  (int)Qt::Key_F21, 47 },
    {  (int)Qt::Key_F22, 48 },
    {  (int)Qt::Key_F23, 49 },
    {  (int)Qt::Key_F24, 50 },
    {  (int)Qt::Key_F25, 51 },
    {  (int)Qt::Key_F26, 52 },
    {  (int)Qt::Key_F27, 53 },
    {  (int)Qt::Key_F28, 54 },
    {  (int)Qt::Key_F29, 55 },
    {  (int)Qt::Key_F30, 56 },
    {  (int)Qt::Key_F31, 57 },
    {  (int)Qt::Key_F32, 58 },
    {  (int)Qt::Key_F33, 59 },
    {  (int)Qt::Key_F34, 60 },
    {  (int)Qt::Key_F35, 61 },
    {  (int)Qt::Key_Super_L, 62 },
    {  (int)Qt::Key_Super_R, 63 },
    {  (int)Qt::Key_Menu, 64 },
    {  (int)Qt::Key_Hyper_L, 65 },
    {  (int)Qt::Key_Hyper_R, 66 },
    {  (int)Qt::Key_Help, 67 },
    {  (int)Qt::Key_Direction_L, 68 },
    {  (int)Qt::Key_Direction_R, 69 },
    {  (int)Qt::Key_Space, 70 },
    {  (int)Qt::Key_Any, 71 },
    {  (int)Qt::Key_Exclam, 72 },
    {  (int)Qt::Key_QuoteDbl, 73 },
    {  (int)Qt::Key_NumberSign, 74 },
    {  (int)Qt::Key_Dollar, 75 },
    {  (int)Qt::Key_Percent, 76 },
    {  (int)Qt::Key_Ampersand, 77 },
    {  (int)Qt::Key_Apostrophe, 78 },
    {  (int)Qt::Key_ParenLeft, 79 },
    {  (int)Qt::Key_ParenRight, 80 },
    {  (int)Qt::Key_Asterisk, 81 },
    {  (int)Qt::Key_Plus, 82 },
    {  (int)Qt::Key_Comma, 83 },
    {  (int)Qt::Key_Minus, 84 },
    {  (int)Qt::Key_Period, 85 },
    {  (int)Qt::Key_Slash, 86 },
    {  (int)Qt::Key_0, 87 },
    {  (int)Qt::Key_1, 88 },
    {  (int)Qt::Key_2, 89 },
    {  (int)Qt::Key_3, 90 },
    {  (int)Qt::Key_4, 91 },
    {  (int)Qt::Key_5, 92 },
    {  (int)Qt::Key_6, 93 },
    {  (int)Qt::Key_7, 94 },
    {  (int)Qt::Key_8, 95 },
    {  (int)Qt::Key_9, 96 },
    {  (int)Qt::Key_Colon, 97 },
    {  (int)Qt::Key_Semicolon, 98 },
    {  (int)Qt::Key_Less, 99 },
    {  (int)Qt::Key_Equal, 100 },
    {  (int)Qt::Key_Greater, 101 },
    {  (int)Qt::Key_Question, 102 },
    {  (int)Qt::Key_At, 103 },
    {  (int)Qt::Key_A, 104 },
    {  (int)Qt::Key_B, 105 },
    {  (int)Qt::Key_C, 106 },
    {  (int)Qt::Key_D, 107 },
    {  (int)Qt::Key_E, 108 },
    {  (int)Qt::Key_F, 109 },
    {  (int)Qt::Key_G, 110 },
    {  (int)Qt::Key_H, 111 },
    {  (int)Qt::Key_I, 112 },
    {  (int)Qt::Key_J, 113 },
    {  (int)Qt::Key_K, 114 },
    {  (int)Qt::Key_L, 115 },
    {  (int)Qt::Key_M, 116 },
    {  (int)Qt::Key_N, 117 },
    {  (int)Qt::Key_O, 118 },
    {  (int)Qt::Key_P, 119 },
    {  (int)Qt::Key_Q, 120 },
    {  (int)Qt::Key_R, 121 },
    {  (int)Qt::Key_S, 122 },
    {  (int)Qt::Key_T, 123 },
    {  (int)Qt::Key_U, 124 },
    {  (int)Qt::Key_V, 125 },
    {  (int)Qt::Key_W, 126 },
    {  (int)Qt::Key_X, 127 },
    {  (int)Qt::Key_Y, 128 },
    {  (int)Qt::Key_Z, 129 },
    {  (int)Qt::Key_BracketLeft, 130 },
    {  (int)Qt::Key_Backslash, 131 },
    {  (int)Qt::Key_BracketRight, 132 },
    {  (int)Qt::Key_AsciiCircum, 133 },
    {  (int)Qt::Key_Underscore, 134 },
    {  (int)Qt::Key_QuoteLeft, 135 },
    {  (int)Qt::Key_BraceLeft, 136 },
    {  (int)Qt::Key_Bar, 137 },
    {  (int)Qt::Key_BraceRight, 138 },
    {  (int)Qt::Key_AsciiTilde, 139 },
    {  (int)Qt::Key_nobreakspace, 140 },
    {  (int)Qt::Key_exclamdown, 141 },
    {  (int)Qt::Key_cent, 142 },
    {  (int)Qt::Key_sterling, 143 },
    {  (int)Qt::Key_currency, 144 },
    {  (int)Qt::Key_yen, 145 },
    {  (int)Qt::Key_brokenbar, 146 },
    {  (int)Qt::Key_section, 147 },
    {  (int)Qt::Key_diaeresis, 148 },
    {  (int)Qt::Key_copyright, 149 },
    {  (int)Qt::Key_ordfeminine, 150 },
    {  (int)Qt::Key_guillemotleft, 151 },
    {  (int)Qt::Key_notsign, 152 },
    {  (int)Qt::Key_hyphen, 153 },
    {  (int)Qt::Key_registered, 154 },
    {  (int)Qt::Key_macron, 155 },
    {  (int)Qt::Key_degree, 156 },
    {  (int)Qt::Key_plusminus, 157 },
    {  (int)Qt::Key_twosuperior, 158 },
    {  (int)Qt::Key_threesuperior, 159 },
    {  (int)Qt::Key_acute, 160 },
    {  (int)Qt::Key_mu, 161 },
    {  (int)Qt::Key_paragraph, 162 },
    {  (int)Qt::Key_periodcentered, 163 },
    {  (int)Qt::Key_cedilla, 164 },
    {  (int)Qt::Key_onesuperior, 165 },
    {  (int)Qt::Key_masculine, 166 },
    {  (int)Qt::Key_guillemotright, 167 },
    {  (int)Qt::Key_onequarter, 168 },
    {  (int)Qt::Key_onehalf, 169 },
    {  (int)Qt::Key_threequarters, 170 },
    {  (int)Qt::Key_questiondown, 171 },
    {  (int)Qt::Key_Agrave, 172 },
    {  (int)Qt::Key_Aacute, 173 },
    {  (int)Qt::Key_Acircumflex, 174 },
    {  (int)Qt::Key_Atilde, 175 },
    {  (int)Qt::Key_Adiaeresis, 176 },
    {  (int)Qt::Key_Aring, 177 },
    {  (int)Qt::Key_AE, 178 },
    {  (int)Qt::Key_Ccedilla, 179 },
    {  (int)Qt::Key_Egrave, 180 },
    {  (int)Qt::Key_Eacute, 181 },
    {  (int)Qt::Key_Ecircumflex, 182 },
    {  (int)Qt::Key_Ediaeresis, 183 },
    {  (int)Qt::Key_Igrave, 184 },
    {  (int)Qt::Key_Iacute, 185 },
    {  (int)Qt::Key_Icircumflex, 186 },
    {  (int)Qt::Key_Idiaeresis, 187 },
    {  (int)Qt::Key_ETH, 188 },
    {  (int)Qt::Key_Ntilde, 189 },
    {  (int)Qt::Key_Ograve, 190 },
    {  (int)Qt::Key_Oacute, 191 },
    {  (int)Qt::Key_Ocircumflex, 192 },
    {  (int)Qt::Key_Otilde, 193 },
    {  (int)Qt::Key_Odiaeresis, 194 },
    {  (int)Qt::Key_multiply, 195 },
    {  (int)Qt::Key_Ooblique, 196 },
    {  (int)Qt::Key_Ugrave, 197 },
    {  (int)Qt::Key_Uacute, 198 },
    {  (int)Qt::Key_Ucircumflex, 199 },
    {  (int)Qt::Key_Udiaeresis, 200 },
    {  (int)Qt::Key_Yacute, 201 },
    {  (int)Qt::Key_THORN, 202 },
    {  (int)Qt::Key_ssharp, 203 },
    {  (int)Qt::Key_division, 204 },
    {  (int)Qt::Key_ydiaeresis, 205 },
    {  (int)Qt::Key_AltGr, 206 },
    {  (int)Qt::Key_Multi_key, 207 },
    {  (int)Qt::Key_Codeinput, 208 },
    {  (int)Qt::Key_SingleCandidate, 209 },
    {  (int)Qt::Key_MultipleCandidate, 210 },
    {  (int)Qt::Key_PreviousCandidate, 211 },
    {  (int)Qt::Key_Mode_switch, 212 },
    {  (int)Qt::Key_Kanji, 213 },
    {  (int)Qt::Key_Muhenkan, 214 },
    {  (int)Qt::Key_Henkan, 215 },
    {  (int)Qt::Key_Romaji, 216 },
    {  (int)Qt::Key_Hiragana, 217 },
    {  (int)Qt::Key_Katakana, 218 },
    {  (int)Qt::Key_Hiragana_Katakana, 219 },
    {  (int)Qt::Key_Zenkaku, 220 },
    {  (int)Qt::Key_Hankaku, 221 },
    {  (int)Qt::Key_Zenkaku_Hankaku, 222 },
    {  (int)Qt::Key_Touroku, 223 },
    {  (int)Qt::Key_Massyo, 224 },
    {  (int)Qt::Key_Kana_Lock, 225 },
    {  (int)Qt::Key_Kana_Shift, 226 },
    {  (int)Qt::Key_Eisu_Shift, 227 },
    {  (int)Qt::Key_Eisu_toggle, 228 },
    {  (int)Qt::Key_Hangul, 229 },
    {  (int)Qt::Key_Hangul_Start, 230 },
    {  (int)Qt::Key_Hangul_End, 231 },
    {  (int)Qt::Key_Hangul_Hanja, 232 },
    {  (int)Qt::Key_Hangul_Jamo, 233 },
    {  (int)Qt::Key_Hangul_Romaja, 234 },
    {  (int)Qt::Key_Hangul_Jeonja, 235 },
    {  (int)Qt::Key_Hangul_Banja, 236 },
    {  (int)Qt::Key_Hangul_PreHanja, 237 },
    {  (int)Qt::Key_Hangul_PostHanja, 238 },
    {  (int)Qt::Key_Hangul_Special, 239 },
    {  (int)Qt::Key_Dead_Grave, 240 },
    {  (int)Qt::Key_Dead_Acute, 241 },
    {  (int)Qt::Key_Dead_Circumflex, 242 },
    {  (int)Qt::Key_Dead_Tilde, 243 },
    {  (int)Qt::Key_Dead_Macron, 244 },
    {  (int)Qt::Key_Dead_Breve, 245 },
    {  (int)Qt::Key_Dead_Abovedot, 246 },
    {  (int)Qt::Key_Dead_Diaeresis, 247 },
    {  (int)Qt::Key_Dead_Abovering, 248 },
    {  (int)Qt::Key_Dead_Doubleacute, 249 },
    {  (int)Qt::Key_Dead_Caron, 250 },
    {  (int)Qt::Key_Dead_Cedilla, 251 },
    {  (int)Qt::Key_Dead_Ogonek, 252 },
    {  (int)Qt::Key_Dead_Iota, 253 },
    {  (int)Qt::Key_Dead_Voiced_Sound, 254 },
    {  (int)Qt::Key_Dead_Semivoiced_Sound, 255 },
    {  (int)Qt::Key_Dead_Belowdot, 256 },
    {  (int)Qt::Key_Dead_Hook, 257 },
    {  (int)Qt::Key_Dead_Horn, 258 },
    {  (int)Qt::Key_Back, 259 },
    {  (int)Qt::Key_Forward, 260 },
    {  (int)Qt::Key_Stop, 261 },
    {  (int)Qt::Key_Refresh, 262 },
    {  (int)Qt::Key_VolumeDown, 263 },
    {  (int)Qt::Key_VolumeMute, 264 },
    {  (int)Qt::Key_VolumeUp, 265 },
    {  (int)Qt::Key_BassBoost, 266 },
    {  (int)Qt::Key_BassUp, 267 },
    {  (int)Qt::Key_BassDown, 268 },
    {  (int)Qt::Key_TrebleUp, 269 },
    {  (int)Qt::Key_TrebleDown, 270 },
    {  (int)Qt::Key_MediaPlay, 271 },
    {  (int)Qt::Key_MediaStop, 272 },
    {  (int)Qt::Key_MediaPrevious, 273 },
    {  (int)Qt::Key_MediaNext, 274 },
    {  (int)Qt::Key_MediaRecord, 275 },
    {  (int)Qt::Key_MediaPause, 276 },
    {  (int)Qt::Key_MediaTogglePlayPause, 277 },
    {  (int)Qt::Key_HomePage, 278 },
    {  (int)Qt::Key_Favorites, 279 },
    {  (int)Qt::Key_Search, 280 },
    {  (int)Qt::Key_Standby, 281 },
    {  (int)Qt::Key_OpenUrl, 282 },
    {  (int)Qt::Key_LaunchMail, 283 },
    {  (int)Qt::Key_LaunchMedia, 284 },
    {  (int)Qt::Key_Launch0, 285 },
    {  (int)Qt::Key_Launch1, 286 },
    {  (int)Qt::Key_Launch2, 287 },
    {  (int)Qt::Key_Launch3, 288 },
    {  (int)Qt::Key_Launch4, 289 },
    {  (int)Qt::Key_Launch5, 290 },
    {  (int)Qt::Key_Launch6, 291 },
    {  (int)Qt::Key_Launch7, 292 },
    {  (int)Qt::Key_Launch8, 293 },
    {  (int)Qt::Key_Launch9, 294 },
    {  (int)Qt::Key_LaunchA, 295 },
    {  (int)Qt::Key_LaunchB, 296 },
    {  (int)Qt::Key_LaunchC, 297 },
    {  (int)Qt::Key_LaunchD, 298 },
    {  (int)Qt::Key_LaunchE, 299 },
    {  (int)Qt::Key_LaunchF, 300 },
    {  (int)Qt::Key_MonBrightnessUp, 301 },
    {  (int)Qt::Key_MonBrightnessDown, 302 },
    {  (int)Qt::Key_KeyboardLightOnOff, 303 },
    {  (int)Qt::Key_KeyboardBrightnessUp, 304 },
    {  (int)Qt::Key_KeyboardBrightnessDown, 305 },
    {  (int)Qt::Key_PowerOff, 306 },
    {  (int)Qt::Key_WakeUp, 307 },
    {  (int)Qt::Key_Eject, 308 },
    {  (int)Qt::Key_ScreenSaver, 309 },
    {  (int)Qt::Key_WWW, 310 },
    {  (int)Qt::Key_Memo, 311 },
    {  (int)Qt::Key_LightBulb, 312 },
    {  (int)Qt::Key_Shop, 313 },
    {  (int)Qt::Key_History, 314 },
    {  (int)Qt::Key_AddFavorite, 315 },
    {  (int)Qt::Key_HotLinks, 316 },
    {  (int)Qt::Key_BrightnessAdjust, 317 },
    {  (int)Qt::Key_Finance, 318 },
    {  (int)Qt::Key_Community, 319 },
    {  (int)Qt::Key_AudioRewind, 320 },
    {  (int)Qt::Key_BackForward, 321 },
    {  (int)Qt::Key_ApplicationLeft, 322 },
    {  (int)Qt::Key_ApplicationRight, 323 },
    {  (int)Qt::Key_Book, 324 },
    {  (int)Qt::Key_CD, 325 },
    {  (int)Qt::Key_Calculator, 326 },
    {  (int)Qt::Key_ToDoList, 327 },
    {  (int)Qt::Key_ClearGrab, 328 },
    {  (int)Qt::Key_Close, 329 },
    {  (int)Qt::Key_Copy, 330 },
    {  (int)Qt::Key_Cut, 331 },
    {  (int)Qt::Key_Display, 332 },
    {  (int)Qt::Key_DOS, 333 },
    {  (int)Qt::Key_Documents, 334 },
    {  (int)Qt::Key_Excel, 335 },
    {  (int)Qt::Key_Explorer, 336 },
    {  (int)Qt::Key_Game, 337 },
    {  (int)Qt::Key_Go, 338 },
    {  (int)Qt::Key_iTouch, 339 },
    {  (int)Qt::Key_LogOff, 340 },
    {  (int)Qt::Key_Market, 341 },
    {  (int)Qt::Key_Meeting, 342 },
    {  (int)Qt::Key_MenuKB, 343 },
    {  (int)Qt::Key_MenuPB, 344 },
    {  (int)Qt::Key_MySites, 345 },
    {  (int)Qt::Key_News, 346 },
    {  (int)Qt::Key_OfficeHome, 347 },
    {  (int)Qt::Key_Option, 348 },
    {  (int)Qt::Key_Paste, 349 },
    {  (int)Qt::Key_Phone, 350 },
    {  (int)Qt::Key_Calendar, 351 },
    {  (int)Qt::Key_Reply, 352 },
    {  (int)Qt::Key_Reload, 353 },
    {  (int)Qt::Key_RotateWindows, 354 },
    {  (int)Qt::Key_RotationPB, 355 },
    {  (int)Qt::Key_RotationKB, 356 },
    {  (int)Qt::Key_Save, 357 },
    {  (int)Qt::Key_Send, 358 },
    {  (int)Qt::Key_Spell, 359 },
    {  (int)Qt::Key_SplitScreen, 360 },
    {  (int)Qt::Key_Support, 361 },
    {  (int)Qt::Key_TaskPane, 362 },
    {  (int)Qt::Key_Terminal, 363 },
    {  (int)Qt::Key_Tools, 364 },
    {  (int)Qt::Key_Travel, 365 },
    {  (int)Qt::Key_Video, 366 },
    {  (int)Qt::Key_Word, 367 },
    {  (int)Qt::Key_Xfer, 368 },
    {  (int)Qt::Key_ZoomIn, 369 },
    {  (int)Qt::Key_ZoomOut, 370 },
    {  (int)Qt::Key_Away, 371 },
    {  (int)Qt::Key_Messenger, 372 },
    {  (int)Qt::Key_WebCam, 373 },
    {  (int)Qt::Key_MailForward, 374 },
    {  (int)Qt::Key_Pictures, 375 },
    {  (int)Qt::Key_Music, 376 },
    {  (int)Qt::Key_Battery, 377 },
    {  (int)Qt::Key_Bluetooth, 378 },
    {  (int)Qt::Key_WLAN, 379 },
    {  (int)Qt::Key_UWB, 380 },
    {  (int)Qt::Key_AudioForward, 381 },
    {  (int)Qt::Key_AudioRepeat, 382 },
    {  (int)Qt::Key_AudioRandomPlay, 383 },
    {  (int)Qt::Key_Subtitle, 384 },
    {  (int)Qt::Key_AudioCycleTrack, 385 },
    {  (int)Qt::Key_Time, 386 },
    {  (int)Qt::Key_Hibernate, 387 },
    {  (int)Qt::Key_View, 388 },
    {  (int)Qt::Key_TopMenu, 389 },
    {  (int)Qt::Key_PowerDown, 390 },
    {  (int)Qt::Key_Suspend, 391 },
    {  (int)Qt::Key_ContrastAdjust, 392 },
    {  (int)Qt::Key_LaunchG, 393 },
    {  (int)Qt::Key_LaunchH, 394 },
    {  (int)Qt::Key_TouchpadToggle, 395 },
    {  (int)Qt::Key_TouchpadOn, 396 },
    {  (int)Qt::Key_TouchpadOff, 397 },
    {  (int)Qt::Key_MicMute, 398 },
    {  (int)Qt::Key_Red, 399 },
    {  (int)Qt::Key_Green, 400 },
    {  (int)Qt::Key_Yellow, 401 },
    {  (int)Qt::Key_Blue, 402 },
    {  (int)Qt::Key_ChannelUp, 403 },
    {  (int)Qt::Key_ChannelDown, 404 },
    {  (int)Qt::Key_Guide, 405 },
    {  (int)Qt::Key_Info, 406 },
    {  (int)Qt::Key_Settings, 407 },
    {  (int)Qt::Key_MicVolumeUp, 408 },
    {  (int)Qt::Key_MicVolumeDown, 409 },
    {  (int)Qt::Key_New, 410 },
    {  (int)Qt::Key_Open, 411 },
    {  (int)Qt::Key_Find, 412 },
    {  (int)Qt::Key_Undo, 413 },
    {  (int)Qt::Key_Redo, 414 },
    {  (int)Qt::Key_MediaLast, 415 },
    {  (int)Qt::Key_Select, 416 },
    {  (int)Qt::Key_Yes, 417 },
    {  (int)Qt::Key_No, 418 },
    {  (int)Qt::Key_Cancel, 419 },
    {  (int)Qt::Key_Printer, 420 },
    {  (int)Qt::Key_Execute, 421 },
    {  (int)Qt::Key_Sleep, 422 },
    {  (int)Qt::Key_Play, 423 },
    {  (int)Qt::Key_Zoom, 424 },
    {  (int)Qt::Key_Exit, 425 },
    {  (int)Qt::Key_Context1, 426 },
    {  (int)Qt::Key_Context2, 427 },
    {  (int)Qt::Key_Context3, 428 },
    {  (int)Qt::Key_Context4, 429 },
    {  (int)Qt::Key_Call, 430 },
    {  (int)Qt::Key_Hangup, 431 },
    {  (int)Qt::Key_Flip, 432 },
    {  (int)Qt::Key_ToggleCallHangup, 433 },
    {  (int)Qt::Key_VoiceDial, 434 },
    {  (int)Qt::Key_LastNumberRedial, 435 },
    {  (int)Qt::Key_Camera, 436 },
    {  (int)Qt::Key_CameraFocus, 437 },
    {  (int)Qt::Key_unknown, 438 },
    };

    for (int i = 0; i < 439; ++i) {
        s_key_lookup[key_vals[i].key] = key_vals[i].val;
    }

    static KeyVal keyboard_modifier_vals[] =
    {{  (int)Qt::NoModifier, 0 },
    {  (int)Qt::ShiftModifier, 1 },
    {  (int)Qt::ControlModifier, 2 },
    {  (int)Qt::AltModifier, 3 },
    {  (int)Qt::MetaModifier, 4 },
    {  (int)Qt::KeypadModifier, 5 },
    {  (int)Qt::GroupSwitchModifier, 6 },
    {  (int)Qt::KeyboardModifierMask, 7 },
    };

    for (int i = 0; i < 8; ++i) {
        s_keyboard_modifier_lookup[keyboard_modifier_vals[i].key] = keyboard_modifier_vals[i].val;
    }

    static KeyVal layout_direction_vals[] =
    {{  (int)Qt::LeftToRight, 0 },
    {  (int)Qt::RightToLeft, 1 },
    {  (int)Qt::LayoutDirectionAuto, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_layout_direction_lookup[layout_direction_vals[i].key] = layout_direction_vals[i].val;
    }

    static KeyVal mask_mode_vals[] =
    {{  (int)Qt::MaskInColor, 0 },
    {  (int)Qt::MaskOutColor, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_mask_mode_lookup[mask_mode_vals[i].key] = mask_mode_vals[i].val;
    }

    static KeyVal match_flag_vals[] =
    {{  (int)Qt::MatchExactly, 0 },
    {  (int)Qt::MatchContains, 1 },
    {  (int)Qt::MatchStartsWith, 2 },
    {  (int)Qt::MatchEndsWith, 3 },
    {  (int)Qt::MatchRegExp, 4 },
    {  (int)Qt::MatchWildcard, 5 },
    {  (int)Qt::MatchFixedString, 6 },
    {  (int)Qt::MatchCaseSensitive, 7 },
    {  (int)Qt::MatchWrap, 8 },
    {  (int)Qt::MatchRecursive, 9 },
    };

    for (int i = 0; i < 10; ++i) {
        s_match_flag_lookup[match_flag_vals[i].key] = match_flag_vals[i].val;
    }

    static KeyVal modifier_vals[] =
    {{  (int)Qt::META, 0 },
    {  (int)Qt::SHIFT, 1 },
    {  (int)Qt::CTRL, 2 },
    {  (int)Qt::ALT, 3 },
    {  (int)Qt::MODIFIER_MASK, 4 },
    {  (int)Qt::UNICODE_ACCEL, 5 },
    };

    for (int i = 0; i < 6; ++i) {
        s_modifier_lookup[modifier_vals[i].key] = modifier_vals[i].val;
    }

    static KeyVal mouse_button_vals[] =
    {{  (int)Qt::NoButton, 0 },
    {  (int)Qt::LeftButton, 1 },
    {  (int)Qt::RightButton, 2 },
    {  (int)Qt::MidButton, 3 },
    {  (int)Qt::MiddleButton, 4 },
    {  (int)Qt::BackButton, 5 },
    {  (int)Qt::XButton1, 6 },
    {  (int)Qt::ExtraButton1, 7 },
    {  (int)Qt::ForwardButton, 8 },
    {  (int)Qt::XButton2, 9 },
    {  (int)Qt::ExtraButton2, 10 },
    {  (int)Qt::TaskButton, 11 },
    {  (int)Qt::ExtraButton3, 12 },
    {  (int)Qt::ExtraButton4, 13 },
    {  (int)Qt::ExtraButton5, 14 },
    {  (int)Qt::ExtraButton6, 15 },
    {  (int)Qt::ExtraButton7, 16 },
    {  (int)Qt::ExtraButton8, 17 },
    {  (int)Qt::ExtraButton9, 18 },
    {  (int)Qt::ExtraButton10, 19 },
    {  (int)Qt::ExtraButton11, 20 },
    {  (int)Qt::ExtraButton12, 21 },
    {  (int)Qt::ExtraButton13, 22 },
    {  (int)Qt::ExtraButton14, 23 },
    {  (int)Qt::ExtraButton15, 24 },
    {  (int)Qt::ExtraButton16, 25 },
    {  (int)Qt::ExtraButton17, 26 },
    {  (int)Qt::ExtraButton18, 27 },
    {  (int)Qt::ExtraButton19, 28 },
    {  (int)Qt::ExtraButton20, 29 },
    {  (int)Qt::ExtraButton21, 30 },
    {  (int)Qt::ExtraButton22, 31 },
    {  (int)Qt::ExtraButton23, 32 },
    {  (int)Qt::ExtraButton24, 33 },
    {  (int)Qt::AllButtons, 34 },
    {  (int)Qt::MaxMouseButton, 35 },
    {  (int)Qt::MouseButtonMask, 36 },
    };

    for (int i = 0; i < 37; ++i) {
        s_mouse_button_lookup[mouse_button_vals[i].key] = mouse_button_vals[i].val;
    }

    static KeyVal mouse_event_flag_vals[] =
    {{  (int)Qt::MouseEventCreatedDoubleClick, 0 },
    {  (int)Qt::MouseEventFlagMask, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_mouse_event_flag_lookup[mouse_event_flag_vals[i].key] = mouse_event_flag_vals[i].val;
    }

    static KeyVal mouse_event_source_vals[] =
    {{  (int)Qt::MouseEventNotSynthesized, 0 },
    {  (int)Qt::MouseEventSynthesizedBySystem, 1 },
    {  (int)Qt::MouseEventSynthesizedByQt, 2 },
    {  (int)Qt::MouseEventSynthesizedByApplication, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_mouse_event_source_lookup[mouse_event_source_vals[i].key] = mouse_event_source_vals[i].val;
    }

    static KeyVal native_gesture_type_vals[] =
    {{  (int)Qt::BeginNativeGesture, 0 },
    {  (int)Qt::EndNativeGesture, 1 },
    {  (int)Qt::PanNativeGesture, 2 },
    {  (int)Qt::ZoomNativeGesture, 3 },
    {  (int)Qt::SmartZoomNativeGesture, 4 },
    {  (int)Qt::RotateNativeGesture, 5 },
    {  (int)Qt::SwipeNativeGesture, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_native_gesture_type_lookup[native_gesture_type_vals[i].key] = native_gesture_type_vals[i].val;
    }

    static KeyVal navigation_mode_vals[] =
    {{  (int)Qt::NavigationModeNone, 0 },
    {  (int)Qt::NavigationModeKeypadTabOrder, 1 },
    {  (int)Qt::NavigationModeKeypadDirectional, 2 },
    {  (int)Qt::NavigationModeCursorAuto, 3 },
    {  (int)Qt::NavigationModeCursorForceVisible, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_navigation_mode_lookup[navigation_mode_vals[i].key] = navigation_mode_vals[i].val;
    }

    static KeyVal orientation_vals[] =
    {{  (int)Qt::Horizontal, 0 },
    {  (int)Qt::Vertical, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_orientation_lookup[orientation_vals[i].key] = orientation_vals[i].val;
    }

    static KeyVal pen_cap_style_vals[] =
    {{  (int)Qt::FlatCap, 0 },
    {  (int)Qt::SquareCap, 1 },
    {  (int)Qt::RoundCap, 2 },
    {  (int)Qt::MPenCapStyle, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_pen_cap_style_lookup[pen_cap_style_vals[i].key] = pen_cap_style_vals[i].val;
    }

    static KeyVal pen_join_style_vals[] =
    {{  (int)Qt::MiterJoin, 0 },
    {  (int)Qt::BevelJoin, 1 },
    {  (int)Qt::RoundJoin, 2 },
    {  (int)Qt::SvgMiterJoin, 3 },
    {  (int)Qt::MPenJoinStyle, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_pen_join_style_lookup[pen_join_style_vals[i].key] = pen_join_style_vals[i].val;
    }

    static KeyVal pen_style_vals[] =
    {{  (int)Qt::NoPen, 0 },
    {  (int)Qt::SolidLine, 1 },
    {  (int)Qt::DashLine, 2 },
    {  (int)Qt::DotLine, 3 },
    {  (int)Qt::DashDotLine, 4 },
    {  (int)Qt::DashDotDotLine, 5 },
    {  (int)Qt::CustomDashLine, 6 },
    {  (int)Qt::MPenStyle, 7 },
    };

    for (int i = 0; i < 8; ++i) {
        s_pen_style_lookup[pen_style_vals[i].key] = pen_style_vals[i].val;
    }

    static KeyVal screen_orientation_vals[] =
    {{  (int)Qt::PrimaryOrientation, 0 },
    {  (int)Qt::PortraitOrientation, 1 },
    {  (int)Qt::LandscapeOrientation, 2 },
    {  (int)Qt::InvertedPortraitOrientation, 3 },
    {  (int)Qt::InvertedLandscapeOrientation, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_screen_orientation_lookup[screen_orientation_vals[i].key] = screen_orientation_vals[i].val;
    }

    static KeyVal scroll_bar_policy_vals[] =
    {{  (int)Qt::ScrollBarAsNeeded, 0 },
    {  (int)Qt::ScrollBarAlwaysOff, 1 },
    {  (int)Qt::ScrollBarAlwaysOn, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_scroll_bar_policy_lookup[scroll_bar_policy_vals[i].key] = scroll_bar_policy_vals[i].val;
    }

    static KeyVal scroll_phase_vals[] =
    {{  (int)Qt::NoScrollPhase, 0 },
    {  (int)Qt::ScrollBegin, 1 },
    {  (int)Qt::ScrollUpdate, 2 },
    {  (int)Qt::ScrollEnd, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_scroll_phase_lookup[scroll_phase_vals[i].key] = scroll_phase_vals[i].val;
    }

    static KeyVal shortcut_context_vals[] =
    {{  (int)Qt::WidgetShortcut, 0 },
    {  (int)Qt::WindowShortcut, 1 },
    {  (int)Qt::ApplicationShortcut, 2 },
    {  (int)Qt::WidgetWithChildrenShortcut, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_shortcut_context_lookup[shortcut_context_vals[i].key] = shortcut_context_vals[i].val;
    }

    static KeyVal size_hint_vals[] =
    {{  (int)Qt::MinimumSize, 0 },
    {  (int)Qt::PreferredSize, 1 },
    {  (int)Qt::MaximumSize, 2 },
    {  (int)Qt::MinimumDescent, 3 },
    {  (int)Qt::NSizeHints, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_size_hint_lookup[size_hint_vals[i].key] = size_hint_vals[i].val;
    }

    static KeyVal size_mode_vals[] =
    {{  (int)Qt::AbsoluteSize, 0 },
    {  (int)Qt::RelativeSize, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_size_mode_lookup[size_mode_vals[i].key] = size_mode_vals[i].val;
    }

    static KeyVal sort_order_vals[] =
    {{  (int)Qt::AscendingOrder, 0 },
    {  (int)Qt::DescendingOrder, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_sort_order_lookup[sort_order_vals[i].key] = sort_order_vals[i].val;
    }

    static KeyVal tab_focus_behavior_vals[] =
    {{  (int)Qt::NoTabFocus, 0 },
    {  (int)Qt::TabFocusTextControls, 1 },
    {  (int)Qt::TabFocusListControls, 2 },
    {  (int)Qt::TabFocusAllControls, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_tab_focus_behavior_lookup[tab_focus_behavior_vals[i].key] = tab_focus_behavior_vals[i].val;
    }

    static KeyVal text_elide_mode_vals[] =
    {{  (int)Qt::ElideLeft, 0 },
    {  (int)Qt::ElideRight, 1 },
    {  (int)Qt::ElideMiddle, 2 },
    {  (int)Qt::ElideNone, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_text_elide_mode_lookup[text_elide_mode_vals[i].key] = text_elide_mode_vals[i].val;
    }

    static KeyVal text_flag_vals[] =
    {{  (int)Qt::TextSingleLine, 0 },
    {  (int)Qt::TextDontClip, 1 },
    {  (int)Qt::TextExpandTabs, 2 },
    {  (int)Qt::TextShowMnemonic, 3 },
    {  (int)Qt::TextWordWrap, 4 },
    {  (int)Qt::TextWrapAnywhere, 5 },
    {  (int)Qt::TextDontPrint, 6 },
    {  (int)Qt::TextIncludeTrailingSpaces, 7 },
    {  (int)Qt::TextHideMnemonic, 8 },
    {  (int)Qt::TextJustificationForced, 9 },
    {  (int)Qt::TextForceLeftToRight, 10 },
    {  (int)Qt::TextForceRightToLeft, 11 },
    {  (int)Qt::TextLongestVariant, 12 },
    {  (int)Qt::TextBypassShaping, 13 },
    };

    for (int i = 0; i < 14; ++i) {
        s_text_flag_lookup[text_flag_vals[i].key] = text_flag_vals[i].val;
    }

    static KeyVal text_format_vals[] =
    {{  (int)Qt::PlainText, 0 },
    {  (int)Qt::RichText, 1 },
    {  (int)Qt::AutoText, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_text_format_lookup[text_format_vals[i].key] = text_format_vals[i].val;
    }

    static KeyVal text_interaction_flag_vals[] =
    {{  (int)Qt::NoTextInteraction, 0 },
    {  (int)Qt::TextSelectableByMouse, 1 },
    {  (int)Qt::TextSelectableByKeyboard, 2 },
    {  (int)Qt::LinksAccessibleByMouse, 3 },
    {  (int)Qt::LinksAccessibleByKeyboard, 4 },
    {  (int)Qt::TextEditable, 5 },
    {  (int)Qt::TextEditorInteraction, 6 },
    {  (int)Qt::TextBrowserInteraction, 7 },
    };

    for (int i = 0; i < 8; ++i) {
        s_text_interaction_flag_lookup[text_interaction_flag_vals[i].key] = text_interaction_flag_vals[i].val;
    }

    static KeyVal tile_rule_vals[] =
    {{  (int)Qt::StretchTile, 0 },
    {  (int)Qt::RepeatTile, 1 },
    {  (int)Qt::RoundTile, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_tile_rule_lookup[tile_rule_vals[i].key] = tile_rule_vals[i].val;
    }

    static KeyVal time_spec_vals[] =
    {{  (int)Qt::LocalTime, 0 },
    {  (int)Qt::UTC, 1 },
    {  (int)Qt::OffsetFromUTC, 2 },
    {  (int)Qt::TimeZone, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_time_spec_lookup[time_spec_vals[i].key] = time_spec_vals[i].val;
    }

    static KeyVal timer_type_vals[] =
    {{  (int)Qt::PreciseTimer, 0 },
    {  (int)Qt::CoarseTimer, 1 },
    {  (int)Qt::VeryCoarseTimer, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_timer_type_lookup[timer_type_vals[i].key] = timer_type_vals[i].val;
    }

    static KeyVal tool_bar_area_vals[] =
    {{  (int)Qt::LeftToolBarArea, 0 },
    {  (int)Qt::RightToolBarArea, 1 },
    {  (int)Qt::TopToolBarArea, 2 },
    {  (int)Qt::BottomToolBarArea, 3 },
    {  (int)Qt::ToolBarArea_Mask, 4 },
    {  (int)Qt::AllToolBarAreas, 5 },
    {  (int)Qt::NoToolBarArea, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_tool_bar_area_lookup[tool_bar_area_vals[i].key] = tool_bar_area_vals[i].val;
    }

    static KeyVal tool_bar_area_sizes_vals[] =
    {{  (int)Qt::NToolBarAreas, 0 },
    };

    for (int i = 0; i < 1; ++i) {
        s_tool_bar_area_sizes_lookup[tool_bar_area_sizes_vals[i].key] = tool_bar_area_sizes_vals[i].val;
    }

    static KeyVal tool_button_style_vals[] =
    {{  (int)Qt::ToolButtonIconOnly, 0 },
    {  (int)Qt::ToolButtonTextOnly, 1 },
    {  (int)Qt::ToolButtonTextBesideIcon, 2 },
    {  (int)Qt::ToolButtonTextUnderIcon, 3 },
    {  (int)Qt::ToolButtonFollowStyle, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_tool_button_style_lookup[tool_button_style_vals[i].key] = tool_button_style_vals[i].val;
    }

    static KeyVal touch_point_state_vals[] =
    {{  (int)Qt::TouchPointPressed, 0 },
    {  (int)Qt::TouchPointMoved, 1 },
    {  (int)Qt::TouchPointStationary, 2 },
    {  (int)Qt::TouchPointReleased, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_touch_point_state_lookup[touch_point_state_vals[i].key] = touch_point_state_vals[i].val;
    }

    static KeyVal transformation_mode_vals[] =
    {{  (int)Qt::FastTransformation, 0 },
    {  (int)Qt::SmoothTransformation, 1 },
    };

    for (int i = 0; i < 2; ++i) {
        s_transformation_mode_lookup[transformation_mode_vals[i].key] = transformation_mode_vals[i].val;
    }

    static KeyVal ui_effect_vals[] =
    {{  (int)Qt::UI_General, 0 },
    {  (int)Qt::UI_AnimateMenu, 1 },
    {  (int)Qt::UI_FadeMenu, 2 },
    {  (int)Qt::UI_AnimateCombo, 3 },
    {  (int)Qt::UI_AnimateTooltip, 4 },
    {  (int)Qt::UI_FadeTooltip, 5 },
    {  (int)Qt::UI_AnimateToolBox, 6 },
    };

    for (int i = 0; i < 7; ++i) {
        s_ui_effect_lookup[ui_effect_vals[i].key] = ui_effect_vals[i].val;
    }

    static KeyVal white_space_mode_vals[] =
    {{  (int)Qt::WhiteSpaceNormal, 0 },
    {  (int)Qt::WhiteSpacePre, 1 },
    {  (int)Qt::WhiteSpaceNoWrap, 2 },
    {  (int)Qt::WhiteSpaceModeUndefined, 3 },
    };

    for (int i = 0; i < 4; ++i) {
        s_white_space_mode_lookup[white_space_mode_vals[i].key] = white_space_mode_vals[i].val;
    }

    static KeyVal widget_attribute_vals[] =
    {{  (int)Qt::WA_Disabled, 0 },
    {  (int)Qt::WA_UnderMouse, 1 },
    {  (int)Qt::WA_MouseTracking, 2 },
    {  (int)Qt::WA_ContentsPropagated, 3 },
    {  (int)Qt::WA_OpaquePaintEvent, 4 },
    {  (int)Qt::WA_NoBackground, 5 },
    {  (int)Qt::WA_StaticContents, 6 },
    {  (int)Qt::WA_LaidOut, 7 },
    {  (int)Qt::WA_PaintOnScreen, 8 },
    {  (int)Qt::WA_NoSystemBackground, 9 },
    {  (int)Qt::WA_UpdatesDisabled, 10 },
    {  (int)Qt::WA_Mapped, 11 },
    {  (int)Qt::WA_MacNoClickThrough, 12 },
    {  (int)Qt::WA_InputMethodEnabled, 13 },
    {  (int)Qt::WA_WState_Visible, 14 },
    {  (int)Qt::WA_WState_Hidden, 15 },
    {  (int)Qt::WA_ForceDisabled, 16 },
    {  (int)Qt::WA_KeyCompression, 17 },
    {  (int)Qt::WA_PendingMoveEvent, 18 },
    {  (int)Qt::WA_PendingResizeEvent, 19 },
    {  (int)Qt::WA_SetPalette, 20 },
    {  (int)Qt::WA_SetFont, 21 },
    {  (int)Qt::WA_SetCursor, 22 },
    {  (int)Qt::WA_NoChildEventsFromChildren, 23 },
    {  (int)Qt::WA_WindowModified, 24 },
    {  (int)Qt::WA_Resized, 25 },
    {  (int)Qt::WA_Moved, 26 },
    {  (int)Qt::WA_PendingUpdate, 27 },
    {  (int)Qt::WA_InvalidSize, 28 },
    {  (int)Qt::WA_MacBrushedMetal, 29 },
    {  (int)Qt::WA_MacMetalStyle, 30 },
    {  (int)Qt::WA_CustomWhatsThis, 31 },
    {  (int)Qt::WA_LayoutOnEntireRect, 32 },
    {  (int)Qt::WA_OutsideWSRange, 33 },
    {  (int)Qt::WA_GrabbedShortcut, 34 },
    {  (int)Qt::WA_TransparentForMouseEvents, 35 },
    {  (int)Qt::WA_PaintUnclipped, 36 },
    {  (int)Qt::WA_SetWindowIcon, 37 },
    {  (int)Qt::WA_NoMouseReplay, 38 },
    {  (int)Qt::WA_DeleteOnClose, 39 },
    {  (int)Qt::WA_RightToLeft, 40 },
    {  (int)Qt::WA_SetLayoutDirection, 41 },
    {  (int)Qt::WA_NoChildEventsForParent, 42 },
    {  (int)Qt::WA_ForceUpdatesDisabled, 43 },
    {  (int)Qt::WA_WState_Created, 44 },
    {  (int)Qt::WA_WState_CompressKeys, 45 },
    {  (int)Qt::WA_WState_InPaintEvent, 46 },
    {  (int)Qt::WA_WState_Reparented, 47 },
    {  (int)Qt::WA_WState_ConfigPending, 48 },
    {  (int)Qt::WA_WState_Polished, 49 },
    {  (int)Qt::WA_WState_DND, 50 },
    {  (int)Qt::WA_WState_OwnSizePolicy, 51 },
    {  (int)Qt::WA_WState_ExplicitShowHide, 52 },
    {  (int)Qt::WA_ShowModal, 53 },
    {  (int)Qt::WA_MouseNoMask, 54 },
    {  (int)Qt::WA_GroupLeader, 55 },
    {  (int)Qt::WA_NoMousePropagation, 56 },
    {  (int)Qt::WA_Hover, 57 },
    {  (int)Qt::WA_InputMethodTransparent, 58 },
    {  (int)Qt::WA_QuitOnClose, 59 },
    {  (int)Qt::WA_KeyboardFocusChange, 60 },
    {  (int)Qt::WA_AcceptDrops, 61 },
    {  (int)Qt::WA_DropSiteRegistered, 62 },
    {  (int)Qt::WA_ForceAcceptDrops, 63 },
    {  (int)Qt::WA_WindowPropagation, 64 },
    {  (int)Qt::WA_NoX11EventCompression, 65 },
    {  (int)Qt::WA_TintedBackground, 66 },
    {  (int)Qt::WA_X11OpenGLOverlay, 67 },
    {  (int)Qt::WA_AlwaysShowToolTips, 68 },
    {  (int)Qt::WA_MacOpaqueSizeGrip, 69 },
    {  (int)Qt::WA_SetStyle, 70 },
    {  (int)Qt::WA_SetLocale, 71 },
    {  (int)Qt::WA_MacShowFocusRect, 72 },
    {  (int)Qt::WA_MacNormalSize, 73 },
    {  (int)Qt::WA_MacSmallSize, 74 },
    {  (int)Qt::WA_MacMiniSize, 75 },
    {  (int)Qt::WA_LayoutUsesWidgetRect, 76 },
    {  (int)Qt::WA_StyledBackground, 77 },
    {  (int)Qt::WA_MSWindowsUseDirect3D, 78 },
    {  (int)Qt::WA_CanHostQMdiSubWindowTitleBar, 79 },
    {  (int)Qt::WA_MacAlwaysShowToolWindow, 80 },
    {  (int)Qt::WA_StyleSheet, 81 },
    {  (int)Qt::WA_ShowWithoutActivating, 82 },
    {  (int)Qt::WA_X11BypassTransientForHint, 83 },
    {  (int)Qt::WA_NativeWindow, 84 },
    {  (int)Qt::WA_DontCreateNativeAncestors, 85 },
    {  (int)Qt::WA_MacVariableSize, 86 },
    {  (int)Qt::WA_DontShowOnScreen, 87 },
    {  (int)Qt::WA_X11NetWmWindowTypeDesktop, 88 },
    {  (int)Qt::WA_X11NetWmWindowTypeDock, 89 },
    {  (int)Qt::WA_X11NetWmWindowTypeToolBar, 90 },
    {  (int)Qt::WA_X11NetWmWindowTypeMenu, 91 },
    {  (int)Qt::WA_X11NetWmWindowTypeUtility, 92 },
    {  (int)Qt::WA_X11NetWmWindowTypeSplash, 93 },
    {  (int)Qt::WA_X11NetWmWindowTypeDialog, 94 },
    {  (int)Qt::WA_X11NetWmWindowTypeDropDownMenu, 95 },
    {  (int)Qt::WA_X11NetWmWindowTypePopupMenu, 96 },
    {  (int)Qt::WA_X11NetWmWindowTypeToolTip, 97 },
    {  (int)Qt::WA_X11NetWmWindowTypeNotification, 98 },
    {  (int)Qt::WA_X11NetWmWindowTypeCombo, 99 },
    {  (int)Qt::WA_X11NetWmWindowTypeDND, 100 },
    {  (int)Qt::WA_MacFrameworkScaled, 101 },
    {  (int)Qt::WA_SetWindowModality, 102 },
    {  (int)Qt::WA_WState_WindowOpacitySet, 103 },
    {  (int)Qt::WA_TranslucentBackground, 104 },
    {  (int)Qt::WA_AcceptTouchEvents, 105 },
    {  (int)Qt::WA_WState_AcceptedTouchBeginEvent, 106 },
    {  (int)Qt::WA_TouchPadAcceptSingleTouchEvents, 107 },
    {  (int)Qt::WA_X11DoNotAcceptFocus, 108 },
    {  (int)Qt::WA_MacNoShadow, 109 },
    {  (int)Qt::WA_AlwaysStackOnTop, 110 },
    {  (int)Qt::WA_TabletTracking, 111 },
    {  (int)Qt::WA_AttributeCount, 112 },
    };

    for (int i = 0; i < 113; ++i) {
        s_widget_attribute_lookup[widget_attribute_vals[i].key] = widget_attribute_vals[i].val;
    }

    static KeyVal window_frame_section_vals[] =
    {{  (int)Qt::NoSection, 0 },
    {  (int)Qt::LeftSection, 1 },
    {  (int)Qt::TopLeftSection, 2 },
    {  (int)Qt::TopSection, 3 },
    {  (int)Qt::TopRightSection, 4 },
    {  (int)Qt::RightSection, 5 },
    {  (int)Qt::BottomRightSection, 6 },
    {  (int)Qt::BottomSection, 7 },
    {  (int)Qt::BottomLeftSection, 8 },
    {  (int)Qt::TitleBarArea, 9 },
    };

    for (int i = 0; i < 10; ++i) {
        s_window_frame_section_lookup[window_frame_section_vals[i].key] = window_frame_section_vals[i].val;
    }

    static KeyVal window_modality_vals[] =
    {{  (int)Qt::NonModal, 0 },
    {  (int)Qt::WindowModal, 1 },
    {  (int)Qt::ApplicationModal, 2 },
    };

    for (int i = 0; i < 3; ++i) {
        s_window_modality_lookup[window_modality_vals[i].key] = window_modality_vals[i].val;
    }

    static KeyVal window_state_vals[] =
    {{  (int)Qt::WindowNoState, 0 },
    {  (int)Qt::WindowMinimized, 1 },
    {  (int)Qt::WindowMaximized, 2 },
    {  (int)Qt::WindowFullScreen, 3 },
    {  (int)Qt::WindowActive, 4 },
    };

    for (int i = 0; i < 5; ++i) {
        s_window_state_lookup[window_state_vals[i].key] = window_state_vals[i].val;
    }

    static KeyVal window_type_vals[] =
    {{  (int)Qt::Widget, 0 },
    {  (int)Qt::Window, 1 },
    {  (int)Qt::Dialog, 2 },
    {  (int)Qt::Sheet, 3 },
    {  (int)Qt::Drawer, 4 },
    {  (int)Qt::Popup, 5 },
    {  (int)Qt::Tool, 6 },
    {  (int)Qt::ToolTip, 7 },
    {  (int)Qt::SplashScreen, 8 },
    {  (int)Qt::Desktop, 9 },
    {  (int)Qt::SubWindow, 10 },
    {  (int)Qt::ForeignWindow, 11 },
    {  (int)Qt::CoverWindow, 12 },
    {  (int)Qt::WindowType_Mask, 13 },
    {  (int)Qt::MSWindowsFixedSizeDialogHint, 14 },
    {  (int)Qt::MSWindowsOwnDC, 15 },
    {  (int)Qt::BypassWindowManagerHint, 16 },
    {  (int)Qt::X11BypassWindowManagerHint, 17 },
    {  (int)Qt::FramelessWindowHint, 18 },
    {  (int)Qt::WindowTitleHint, 19 },
    {  (int)Qt::WindowSystemMenuHint, 20 },
    {  (int)Qt::WindowMinimizeButtonHint, 21 },
    {  (int)Qt::WindowMaximizeButtonHint, 22 },
    {  (int)Qt::WindowMinMaxButtonsHint, 23 },
    {  (int)Qt::WindowContextHelpButtonHint, 24 },
    {  (int)Qt::WindowShadeButtonHint, 25 },
    {  (int)Qt::WindowStaysOnTopHint, 26 },
    {  (int)Qt::WindowTransparentForInput, 27 },
    {  (int)Qt::WindowOverridesSystemGestures, 28 },
    {  (int)Qt::WindowDoesNotAcceptFocus, 29 },
    {  (int)Qt::MaximizeUsingFullscreenGeometryHint, 30 },
    {  (int)Qt::CustomizeWindowHint, 31 },
    {  (int)Qt::WindowStaysOnBottomHint, 32 },
    {  (int)Qt::WindowCloseButtonHint, 33 },
    {  (int)Qt::MacWindowToolBarButtonHint, 34 },
    {  (int)Qt::BypassGraphicsProxyWidget, 35 },
    {  (int)Qt::NoDropShadowWindowHint, 36 },
    {  (int)Qt::WindowFullscreenButtonHint, 37 },
    };

    for (int i = 0; i < 38; ++i) {
        s_window_type_lookup[window_type_vals[i].key] = window_type_vals[i].val;
    }
}
