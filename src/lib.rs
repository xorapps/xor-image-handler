mod image_type;
pub use image_type::*;

mod reader;
pub use reader::*;

//TODO
// - image size limitations
// - image limitations on clients

#[cfg(test)]
mod sanity_tests {
    use crate::*;

    #[test] //TODO Write better tests
    fn reader_writer() {
        smol::block_on(async {
            let mut image_files = ImageReader::new();
            let outcome = image_files
                .add_file_path("test_images/001.png")
                .from_kib(200)
                .get_images()
                .await
                .unwrap();

            println!("{:?}", &outcome);

            let foo = outcome[0].clone();

            image_files
                .write_to_file(foo.file_stem(), foo.extension(), foo.bytes())
                .await
                .unwrap();
        })
    }
}
