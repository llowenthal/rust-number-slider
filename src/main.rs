use qmetaobject::prelude::*;

#[derive(QObject, Default)]
struct Counter {
    base: qt_base_class!(trait QObject),

    counter: qt_property!(i32; NOTIFY counter_changed),
    counter_changed: qt_signal!(),

    increment: qt_method!(fn increment(&mut self) {
        self.counter += 1;
        self.counter_changed();
    }),

    decrement: qt_method!(fn decrement(&mut self){
        self.counter -= 1;
        self.counter_changed();
    }),

    set_counter: qt_method!(fn set_counter(&mut self, value: i32) {
        if self.counter != value {
            self.counter = value;
            self.counter_changed();
        }
    }),

}

fn main() {
    let mut engine = QmlEngine::new();

    // Put the QObject in a QObjectBox so it can't move (Qt requires this).
    let counter_box = QObjectBox::new(Counter::default());
    let counter_pinned = counter_box.pinned();

    // Initialize from Rust.
    {
        let mut c = counter_pinned.borrow_mut();
        c.counter = 42;
        c.counter_changed(); // notify QML that the value is set
    }

    // Expose it to QML as a context property named "counterObj"
    engine.set_object_property("counterObj".into(), counter_pinned);

    engine.load_file("qml/main.qml".into());
    engine.exec();

    // IMPORTANT: counter_box must stay alive until after exec() returns.
    // Keeping it in this scope does that.
}
