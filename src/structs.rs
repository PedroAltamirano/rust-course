fn main() {
    struct Student { name: String, level: u8, remote: bool }

    struct Grades(char, char, char, char, f32);

    struct Unit;

    enum WebEvent {
        WELoad,
        WEKeys(String, char),
        WEClick { x: i64, y: i64 }
    }


    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    struct KeyPress(String, char);
    struct MouseClick { x: i64, y: i64 }
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    let we_load = WebEvent::WELoad(true);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKeys(keys);

}
