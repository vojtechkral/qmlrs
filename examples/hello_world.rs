extern crate qmlrs;

#[allow(unused_must_use)]
fn main() {
    let mut view = qmlrs::View::new();

    view.set_source("file:///home/cyndis/src/qmlrs/examples/hello.qml");
    view.show();

    let handle = view.handle();
    view.register_slot("hello".into_string(), box move || {
        let foo = handle.invoke("hello", &[qmlrs::Variant::Int(555)]).unwrap();
        println!("QML hello slot returned: {}", foo);
        qmlrs::Variant::Int(42)
    });

    view.exec();
}
