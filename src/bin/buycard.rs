use enigo::*;
use tokio::time::{sleep, Duration};

async fn wait(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    wait(3000).await;

    let mut count = 0;
    while count <= 2 {
        wait(300).await;

        enigo.move_mouse(726, 311);
        wait(100).await;
        enigo.butto(MouseButton::Left);
        wait(100).await;
        enigo.mouse_move_to(1481, 942);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;

        enigo.mouse_move_to(731, 453);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;
        enigo.mouse_move_to(1481, 942);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;

        enigo.mouse_move_to(734, 593);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;
        enigo.mouse_move_to(1481, 942);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;

        enigo.mouse_move_to(743, 704);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;
        enigo.mouse_move_to(1481, 942);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;

        enigo.mouse_move_to(740, 843);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;
        enigo.mouse_move_to(1481, 942);
        wait(100).await;
        enigo.mouse_click(MouseButton::Left);
        wait(100).await;

        count += 1;
    }
}