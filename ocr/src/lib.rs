use std::error::Error;
use tokio::sync::oneshot;
use std::thread;

pub fn ocr(params: [&str; 2]) -> Result<(), Box<dyn Error>> {
    let s = tesseract::ocr(&params[0], &params[1])?;
    println!(" the result text is {:?}", s);

    Ok(())
}

pub async fn ocr_async(params: [String; 2]) -> Result<(), Box<dyn Error + Send+ Sync>> {
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let s = tesseract::ocr(&params[0], &params[1])?;
        let _ = tx.send(s);
        Ok::<(), Box<dyn Error + Send+ Sync>>(())
    });
    let _ = rx.await;
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_text_from_sample_image() -> Result<(), Box<dyn Error>> {
        let _ = ocr(["data/string.png", "eng"])?;
        let _ = ocr(["data/horizontal_text.png", "eng"])?;
        let _ = ocr(["data/vertical_text.png", "eng"])?;
        Ok(())
    }

    #[tokio::test]
    async fn should_extract_text_from_sample_image_async() -> Result<(), Box<dyn Error + Send + Sync >> {
        let _ = ocr_async(["data/string.png".into(), "eng".into()]).await?;
        let _ = ocr_async(["data/horizontal_text.png".into(), "eng".into()]).await?;
        let _ = ocr_async(["data/vertical_text.png".into(), "eng".into()]).await?;
        Ok(())
    }
}
