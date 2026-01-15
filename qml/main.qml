import QtQuick 2.6
import QtQuick.Window 2.0
import QtQuick.Controls 2.0



Window {
    visible: true
    width: 420
    height: 240
    title: "Rust + Qt Counter"

    SystemPalette { id: pal }
    color: pal.window

    Column {
        anchors.centerIn: parent
        spacing: 16
        width: parent.width - 40

        Text {
            text: counterObj.counter
            font.pointSize: 32
            color: pal.windowText
            horizontalAlignment: Text.AlignHCenter
            width: parent.width
        }

        Row {
            spacing: 16
            anchors.horizontalCenter: parent.horizontalCenter

            Button {
                text: "Decrement"
                enabled: counterObj.counter >= 30
                onClicked: counterObj.decrement()
            }

            Button {
                text: "Increment"
                enabled: counterObj.counter <= 50
                onClicked: counterObj.increment()
            }

        }

      
            
            
        Slider {
            id: counterSlider
            from: 29
            to: 51
            stepSize: 1
            orientation: Qt.Horizontal
            width: parent.width

            // Keep the knob aligned with the backend value
            value: counterObj.counter

            // Update Rust as the user drags (real-time)
            onValueChanged: counterObj.set_counter(Math.round(value))
        }
        

        
    }
}
