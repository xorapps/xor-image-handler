### XOR-IMAGE-HANDLER
A library to handle reading images and generating their MIME and encoding to various encoding schemes like Base64 and Z85

#### Example
```rs
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

```