use std::collections::HashMap;

fn morse_map() -> HashMap<char, &'static str> {
    let mut morse_map = HashMap::new();
    morse_map.insert('a', ".-");
    morse_map.insert('b', "-...");
    morse_map.insert('c', "-.-.");
    morse_map.insert('d', "-..");
    morse_map.insert('e', ".");
    morse_map.insert('f', "..-.");
    morse_map.insert('g', "--.");
    morse_map.insert('h', "....");
    morse_map.insert('i', "..");
    morse_map.insert('j', ".---");
    morse_map.insert('k', "-.-");
    morse_map.insert('l', ".-..");
    morse_map.insert('m', "--");
    morse_map.insert('n', "-.");
    morse_map.insert('o', "---");
    morse_map.insert('p', ".--.");
    morse_map.insert('q', "--.-");
    morse_map.insert('r', ".-.");
    morse_map.insert('s', "...");
    morse_map.insert('t', "-");
    morse_map.insert('u', "..-");
    morse_map.insert('v', "...-");
    morse_map.insert('w', ".--");
    morse_map.insert('x', "-..-");
    morse_map.insert('y', "-.--");
    morse_map.insert('z', "--..");
    morse_map.insert('1', ".----");
    morse_map.insert('2', "..---");
    morse_map.insert('3', "...--");
    morse_map.insert('4', "....-");
    morse_map.insert('5', ".....");
    morse_map.insert('6', "-....");
    morse_map.insert('7', "--...");
    morse_map.insert('8', "---..");
    morse_map.insert('9', "----.");
    morse_map.insert('0', "-----");
    morse_map.insert(' ', " ");
    morse_map
}