/*
Let's model a real-time chat system where users can
share audio and video files.

Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.

Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.

Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".

Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.

In `main`, create a ChatMessage with `content` set to a
string slice.

Create another ChatMessage with `content` set to a String.

Create another ChatMessage with `content' set to a
DigitalContent variant.

Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.

Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

use chrono::{DateTime, Utc};
#[derive(Debug)]
enum DigitalContent {
    _AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

// concrete type implementation for the enum
impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching: {:?}", self.content);
    }
}

// for the generic implementation
// i moved the new method inside this impl block
// since this is sort of blobal
// and should construct with any data type
impl<T> ChatMessage<T> {
    fn new(content: T) -> Self {
        let now: DateTime<Utc> = Utc::now();
        let datetime = now.to_rfc3339();
        ChatMessage {
            content,
            time: datetime,
        }
    }

    fn retrieve_time(&self) -> String {
        self.time.clone() // the reason why we clone the time is because String does not implement
    }
}

fn main() {
    let str_message: ChatMessage<&str> = ChatMessage::new("I am texting you");
    let string_message: ChatMessage<String> = ChatMessage::new(String::from("Yo, what's up dude?"));
    let digital_content_message: ChatMessage<DigitalContent> =
        ChatMessage::new(DigitalContent::VideoFile);

    digital_content_message.consume_entertainment();

    println!("{}", str_message.retrieve_time());
    println!("{}", string_message.retrieve_time());
    println!("{}", digital_content_message.retrieve_time());
}
