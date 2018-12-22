////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QFont>
#include "font_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_swap(struct RUBase* self_c, struct RUBase* other) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->swap(*((WRFont*)other));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_family(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->family();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_family(struct RUBase* self_c, const char* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setFamily(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_style_name(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->styleName();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_style_name(struct RUBase* self_c, const char* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setStyleName(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int font_point_size(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->pointSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_point_size(struct RUBase* self_c, int arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setPointSize(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float font_point_size_f(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->pointSizeF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_point_size_f(struct RUBase* self_c, float arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setPointSizeF(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int font_pixel_size(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->pixelSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_pixel_size(struct RUBase* self_c, int arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setPixelSize(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int font_weight(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->weight();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_weight(struct RUBase* self_c, int arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setWeight(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_bold(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->bold();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_bold(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setBold(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_italic(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->italic();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_italic(struct RUBase* self_c, bool b) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setItalic(b);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_underline(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->underline();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_underline(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setUnderline(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_overline(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->overline();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_overline(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setOverline(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_fixed_pitch(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->fixedPitch();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_fixed_pitch(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setFixedPitch(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_kerning(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->kerning();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_kerning(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setKerning(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t font_style_hint(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->styleHint();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_style_hint(struct RUBase* self_c, uint32_t arg0, uint32_t arg1) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setStyleHint((QFont::StyleHint)arg0, (QFont::StyleStrategy)arg1);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float font_letter_spacing(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->letterSpacing();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t font_letter_spacing_type(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->letterSpacingType();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_letter_spacing(struct RUBase* self_c, uint32_t stype, float spacing) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setLetterSpacing((QFont::SpacingType)stype, spacing);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float font_word_spacing(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->wordSpacing();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_word_spacing(struct RUBase* self_c, float spacing) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setWordSpacing(spacing);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_capitalization(struct RUBase* self_c, uint32_t arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setCapitalization((QFont::Capitalization)arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t font_capitalization(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->capitalization();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_hinting_preference(struct RUBase* self_c, uint32_t hinting_preference) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setHintingPreference((QFont::HintingPreference)hinting_preference);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t font_hinting_preference(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->hintingPreference();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_raw_mode(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->rawMode();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_raw_mode(struct RUBase* self_c, bool arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setRawMode(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_exact_match(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->exactMatch();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool font_is_copy_of(struct RUBase* self_c, struct RUBase* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->isCopyOf(*((WRFont*)arg0));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_set_raw_name(struct RUBase* self_c, const char* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->setRawName(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_raw_name(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->rawName();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_key(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->key();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_substitute(struct RUBase* self_c, const char* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->substitute(QString::fromUtf8(arg0));
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_insert_substitution(struct RUBase* self_c, const char* arg0, const char* arg1) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->insertSubstitution(QString::fromUtf8(arg0), QString::fromUtf8(arg1));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_remove_substitutions(struct RUBase* self_c, const char* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->removeSubstitutions(QString::fromUtf8(arg0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_initialize(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->initialize();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_cleanup(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->cleanup();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_cache_statistics(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->cacheStatistics();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_default_family(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->defaultFamily();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_last_resort_family(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->lastResortFamily();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* font_last_resort_font(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->lastResortFont();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont font_resolve(struct RUBase* self_c, struct RUBase* arg0) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->resolve(*((WRFont*)arg0));
    WRFont* new_val = new WRFont();
    *new_val = ret_value;
    struct RUFont ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint64_t font_resolve_2(struct RUBase* self_c) {
    WRFont* qt_value = (WRFont*)self_c;
    auto ret_value = qt_value->resolve();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void font_resolve_3(struct RUBase* self_c, uint64_t mask) {
    WRFont* qt_value = (WRFont*)self_c;
    qt_value->resolve(mask);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont create_font(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUFont, WRFont>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_font(struct RUBase* priv_data) {
    destroy_generic<WRFont>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont get_font(struct RUBase* priv_data) {
    (void)priv_data;
    RUFont ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUFontFuncs s_font_funcs = {
    destroy_font,
    font_swap,
    font_family,
    font_set_family,
    font_style_name,
    font_set_style_name,
    font_point_size,
    font_set_point_size,
    font_point_size_f,
    font_set_point_size_f,
    font_pixel_size,
    font_set_pixel_size,
    font_weight,
    font_set_weight,
    font_bold,
    font_set_bold,
    font_italic,
    font_set_italic,
    font_underline,
    font_set_underline,
    font_overline,
    font_set_overline,
    font_fixed_pitch,
    font_set_fixed_pitch,
    font_kerning,
    font_set_kerning,
    font_style_hint,
    font_set_style_hint,
    font_letter_spacing,
    font_letter_spacing_type,
    font_set_letter_spacing,
    font_word_spacing,
    font_set_word_spacing,
    font_set_capitalization,
    font_capitalization,
    font_set_hinting_preference,
    font_hinting_preference,
    font_raw_mode,
    font_set_raw_mode,
    font_exact_match,
    font_is_copy_of,
    font_set_raw_name,
    font_raw_name,
    font_key,
    font_substitute,
    font_insert_substitution,
    font_remove_substitutions,
    font_initialize,
    font_cleanup,
    font_cache_statistics,
    font_default_family,
    font_last_resort_family,
    font_last_resort_font,
    font_resolve,
    font_resolve_2,
    font_resolve_3,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUFontAllFuncs s_font_all_funcs = {
    &s_font_funcs,
};

