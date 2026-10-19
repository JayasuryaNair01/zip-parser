use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("test.zip").expect("couldnt open the file");
    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("couldnt read file");

    let ecd_marker = [0x50, 0x4B, 0x05, 0x06];

    let mut found_at = None;

    if data.len() >= 4 {
        for i in (0..=data.len() - 4).rev() {
            if data[i..i + 4] == ecd_marker {
                found_at = Some(i);
                break;
            }
        }
    }
    if let Some(ecd_position) = found_at {
        println!("position for ecd is at {}", ecd_position);

        let position_nof_ecd = ecd_position + 10;
        let nof_ecd = u16::from_le_bytes([data[position_nof_ecd], data[position_nof_ecd + 1]]);
        println!(" no of files in this zip file {}", nof_ecd);

        let position_position_info_cd_in_ed = ecd_position + 16;
        let position_info_cd_in_ed = u32::from_le_bytes([
            data[position_position_info_cd_in_ed],
            data[position_position_info_cd_in_ed + 1],
            data[position_position_info_cd_in_ed + 2],
            data[position_position_info_cd_in_ed + 3],
        ]);
        println!(" position of cental directory {}", position_info_cd_in_ed);

        let cd_marker = [0x50, 0x4B, 0x01, 0x02];
        let cd_position = position_info_cd_in_ed as usize;

        if data.len() >= cd_position + 4 && data[cd_position..cd_position + 4] == cd_marker {
            println!("position for cd is at {}", cd_position);

            let position_file_name_length = cd_position + 28;
            let file_name_length = u16::from_le_bytes([
                data[position_file_name_length],
                data[position_file_name_length + 1],
            ]) as usize;
            println!("the length of the file is {}", file_name_length);

            let file_name_start = cd_position + 46;
            let file_name_end = file_name_start + file_name_length;

            if file_name_end <= data.len() {
                let file_name_bytes = &data[file_name_start..file_name_end];
                let file_name = String::from_utf8_lossy(file_name_bytes);

                println!(" file no 1 {}", file_name);

                let position_compressed_size_start = cd_position + 20;
                let compressed_size = u32::from_le_bytes([
                    data[position_compressed_size_start],
                    data[position_compressed_size_start + 1],
                    data[position_compressed_size_start + 2],
                    data[position_compressed_size_start + 3],
                ]);
                println!("size of the compressed version: {}", compressed_size);

                let position_uncompresed_size_start = cd_position + 24;
                let uncompressed_size = u32::from_le_bytes([
                    data[position_uncompresed_size_start],
                    data[position_uncompresed_size_start + 1],
                    data[position_uncompresed_size_start + 2],
                    data[position_uncompresed_size_start + 3],
                ]);
                println!("size of the uncompressed version: {}", uncompressed_size);

                let position_local_header_start = cd_position + 42;
                let local_header_start = u32::from_le_bytes([
                    data[position_local_header_start],
                    data[position_local_header_start + 1],
                    data[position_local_header_start + 2],
                    data[position_local_header_start + 3],
                ]);
                println!("psoition of local header start {}", local_header_start);

                let local_start = local_header_start as usize;

                let position_filenamelen_local = local_start + 26;
                let filenamelen_local = u16::from_le_bytes([
                    data[position_filenamelen_local],
                    data[position_filenamelen_local + 1],
                ]) as usize;

                let position_local_extra_len = local_start + 28;
                let local_extra_len = u16::from_le_bytes([
                    data[position_local_extra_len],
                    data[position_local_extra_len + 1],
                ]) as usize;

                let data_start = local_start + 30 + filenamelen_local + local_extra_len;
                let data_end = data_start + compressed_size as usize;

                if data_end <= data.len() {
                    let file_data = &data[data_start..data_end];
                    std::fs::write("extracted_test.txt", file_data).expect("could not write file");
                    println!("Extracted file to extracted_test.txt");
                } else {
                    println!("not enough data to extract");
                }
            }
        }
    }
}
