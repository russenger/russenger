# Russenger - Facebook Messenger Webhook Handling in Rust

![Russenger Logo](./image.png)

Russenger is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a
convenient way to construct and send various types of responses, including text messages, quick replies, generic
templates, and media attachments.

## Features

Russenger provides the following features:

- **Text messages:** Send text messages to users.
- **Quick replies:** Send quick replies with buttons to users.
- **Generic templates:** Send generic templates with images, titles, and buttons to users.
- **Media attachments:** Send media attachments such as images, audio, and video to users.
- **Webhook verification:** Verify incoming webhook requests from Facebook.

## Installation

To use Russenger in your Rust project, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
russenger = { git = "https://github.com/j03-dev/russenger", branch = "actix" }
actix = "4"
```

Ensure that you have set up your `.env` file within your project directory. The file should contain the following
configurations:

```env
VERIFY_TOKEN=<your-verify-token>
API=https://graph.facebook.com/v16.0/me/messages?access-token=
PAGE_ACCESS_TOKEN=<your-page-access-token>
HOST=0.0.0.0
PORT=8000
```

#### postgres

```env
DATABASE=postgres://<user>:<password>@<host>/<db_name>
```

#### mysql

```env
DATABASE=mysql://<user>:<password>@<host>/<db_name>
```

#### sqlite

```env
DATABASE=sqlite:<dbname>
```

### Create Static Directory

```bash
mkdir <project>/static/
touch <project>/static/.keep
```

## Usage

### Example Application

The following example demonstrates the usage of Russenger for creating a chatbot in Rust. It includes actions
named `Main`, `Option1`, and `Option2`, along with a user scenario:

```rust
use russenger::{
    Data, Req, Res
    generic::{GenericButton, GenericElement, GenericModel};
    payload::{ActionPayload, Payload};
    quick_replies::{QuickReplie, QuickReplieModel};
    text::TextModel;
    create_action, russenger_app
};


create_action!(Main, |res: Res, req: Req| async move {
    // Welcome message
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!")).await;

    // Example with Quick Replies
    let quick_replies = vec![
        QuickReplie::new(
            "Option 1",
            "",
            Payload::new(
                Option1,
                Some(Data::new("payload_for_option1", None))
            ),
        ),
        QuickReplie::new(
            "Option 2",
            "",
            Payload::new(
                Option2,
                Some(Data::new("payload_for_option2", None))
            ),
        ),
    ];

    res.send(QuickReplieModel::new(
        &req.user,
        "Choose an option:",
        quick_replies,
    ))
    .await;
});

// For Option1
create_action!(Option1, |res: Res, req: Req| async move {
    // Handle Option 1 with a TextModel
    let value: String = req.data.get_value();
    let text = TextModel::new(&req.user, &format!("You selected Option 1 with payload: {}", value));
    res.send(text).await;
});

// For Option2
create_action!(Option2, |res: Res, req: Req| async move {
    // Handle Option 2 with a TextModel
    let value: String = req.data.get_value();
    let text = TextModel::new(&req.user, &format!("You selected Option 2 with payload: {}", value));
    res.send(text).await;

    // Handle Option 2 with a Generic Template
    let generic_elements = vec![GenericElement::new(
        "Option 2",
        "https://example.com/option2.jpg",
        "Option 2 description",
        vec![GenericButton::new(
            "Choose Option 2",
            Payload::new(Main, None),
        )],
    )];

    res.send(GenericModel::new(&req.user, generic_elements))
        .await;
});

russenger_app!(Main, Option1, Option2);
```

### Endpoints

- **GET `/webhook`:** Verify your chatbot with Facebook Messenger. Facebook will send a challenge, and your bot must
  respond correctly for verification.

- **POST `/webhook`:** This is where Facebook Messenger sends messages from users. Handle incoming messages and respond
  accordingly here.

### License

Russenger is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

For more information, visit the [GitHub repository](https://github.com/j03-dev/russenger).

If you have any questions or need assistance, feel free to open an issue or contact the project maintainers.
