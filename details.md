## Rute

Lets look in detail how the generation works

First we have the [api def file](https://github.com/emoon/Rute/blob/master/generator/src/api.def) that is used as input for the generator. In this show case a small struct is used to show how the generation work. Given the following input

```Rust
struct Widget {
    resize(i32 width, i32 height),
}
```

C output

```C
struct RUWidgetFuncs {
    void (*resize)(struct RUBase* self_c, int width, int height);
};

struct RUWidget {
    struct RUWidgetFuncs* funcs;
    struct RUBase* priv_data;
};

#define RUWidget_resize(obj, width, height) obj.funcs->resize(obj.priv_data, width, height)
```

Rust FFI output

```Rust
#[repr(C)]
pub struct RUWidgetFuncs {
    pub resize: extern "C" fn(self_c: *const RUBase, width: i32, height: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidget {
    pub funcs: *const RUWidgetFuncs,
    pub privd: *const RUBase,
}
```

C++ output (WRWidget is a QWidget)

```C
static void widget_resize(struct RUBase* self_c, int width, int height) {
    WRWidget* qt_data = (WRWidget*)self_c;
    qt_data->resize(width, height);
}

struct RUWidgetFuncs s_widget_funcs = {
    widget_resize,
};

static struct RUWidget create_widget(struct RUBase* priv_data) {
    return create_widget_func<struct RUWidget, struct RUWidgetFuncs, WRWidget>(&s_widget_funcs, priv_data);
}

static struct RU s_pu = {
    create_widget,
};

extern "C" struct RU* rute_get() { return (RU*)&s_ru; }

```

Rust output

```Rust
#[derive(Clone)]
pub struct Widget {
    pub obj: Option<RUWidget>,
}

impl Widget {
    pub fn resize (&self, width: i32, height: i32) {
        unsafe {
            let obj = self.obj.unwrap();
            ((*obj.funcs).resize)(obj.privd, width, height);
        }
    }
}

impl Ui {
    pub fn new(pu: *const RU) -> Ui { Ui { pu } }
    pub fn create_widget(&self) -> Widget {
        Widget { obj: Some(unsafe { ((*self.pu).create_widget)((*self.pu).privd) }) }
    }
}
```

Notice that the output showed here has been reduced slighly but should get the idea accross.

### Signals and slots


