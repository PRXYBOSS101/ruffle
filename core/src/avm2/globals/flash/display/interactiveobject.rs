//! `flash.display.InteractiveObject` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::Multiname;
use crate::avm2::Namespace;
use crate::avm2::QName;
use crate::display_object::{TDisplayObject, TInteractiveObject};
use crate::{avm2_stub_getter, avm2_stub_setter};
use gc_arena::GcCell;

/// Implements `flash.display.InteractiveObject`'s instance constructor.
pub fn instance_init<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Err("You cannot directly construct InteractiveObject.".into())
}

/// Implements `flash.display.InteractiveObject`'s native instance constructor.
pub fn native_instance_init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.display.InteractiveObject`'s class constructor.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.mouseEnabled`'s getter.
pub fn mouse_enabled<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.mouse_enabled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.mouseEnabled`'s setter.
pub fn set_mouse_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_boolean();
        int.set_mouse_enabled(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.doubleClickEnabled`'s getter.
pub fn double_click_enabled<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.double_click_enabled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.doubleClickEnabled`'s setter.
pub fn set_double_click_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_boolean();
        int.set_double_click_enabled(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.contextMenu`'s getter.
fn context_menu<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.context_menu());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.contextMenu`'s setter.
fn set_context_menu<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let cls_name = Multiname::new(
            Namespace::package("flash.display", activation.context.gc_context),
            "NativeMenu",
        );
        let cls = activation.resolve_class(&cls_name)?;
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_type(activation, cls)?;
        int.set_context_menu(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

pub fn tab_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.InteractiveObject", "tabEnabled");

    Ok(false.into())
}

pub fn set_tab_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.display.InteractiveObject", "tabIndex");

    Ok(Value::Undefined)
}

pub fn tab_index<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.InteractiveObject", "tabIndex");

    Ok((-1).into())
}

pub fn set_tab_index<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.display.InteractiveObject", "tabIndex");

    Ok(Value::Undefined)
}

pub fn focus_rect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.InteractiveObject", "focusRect");
    Ok(Value::Null)
}

pub fn set_focus_rect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    // NOTE: all values other than true or null are converted to false. (false/null do differ)

    // let's only warn on true, as games sometimes just set focusRect to false for some reason.
    if matches!(args.get(0), Some(Value::Bool(true))) {
        avm2_stub_setter!(activation, "flash.display.InteractiveObject", "focusRect");
    }

    Ok(Value::Null)
}

/// Construct `InteractiveObject`'s class.
pub fn create_class<'gc>(activation: &mut Activation<'_, 'gc>) -> GcCell<'gc, Class<'gc>> {
    let mc = activation.context.gc_context;
    let class = Class::new(
        QName::new(Namespace::package("flash.display", mc), "InteractiveObject"),
        Some(Multiname::new(
            Namespace::package("flash.display", mc),
            "DisplayObject",
        )),
        Method::from_builtin(
            instance_init,
            "<InteractiveObject instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<InteractiveObject class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_native_instance_init(Method::from_builtin(
        native_instance_init,
        "<InteractiveObject native instance initializer>",
        mc,
    ));

    const PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl>,
        Option<NativeMethodImpl>,
    )] = &[
        ("mouseEnabled", Some(mouse_enabled), Some(set_mouse_enabled)),
        (
            "doubleClickEnabled",
            Some(double_click_enabled),
            Some(set_double_click_enabled),
        ),
        ("contextMenu", Some(context_menu), Some(set_context_menu)),
        ("tabEnabled", Some(tab_enabled), Some(set_tab_enabled)),
        ("tabIndex", Some(tab_index), Some(set_tab_index)),
        ("focusRect", Some(focus_rect), Some(set_focus_rect)),
    ];
    write.define_builtin_instance_properties(
        mc,
        activation.avm2().public_namespace,
        PUBLIC_INSTANCE_PROPERTIES,
    );

    class
}
