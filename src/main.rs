#![feature(array_chunks)]

use std::env::args;
use std::fs::{create_dir, File};
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

const START: &[u8; 20] =
    b"\x06\x00\x00\x00\xA1\x00\x00\x00\x00\x00\x01\x00\x01\x00\x00\x00\x86\x80\x00\x00";
const END: &[u8; 5] = b"\x02\xFC\x02\x00\x00";
const END_IML: &[u8; 5] = b"\x02\xFC\x03\x00\x00";

fn main() {
    let path = args().nth(1).expect("missing path arg");

    let mut buf = vec![];
    File::open(path)
        .expect("failed to open")
        .read_to_end(&mut buf)
        .expect("couldn't read file");
    let buf = buf;

    let out = Path::new("out");
    if out
        .metadata()
        .is_err_and(|err| err.kind() == ErrorKind::NotFound)
    {
        create_dir(out).expect("could not create out dir");
    }

    let mut addr = 0;

    let mut current_start = 0;
    let mut next_start = 0;

    let mut count = 0;
    let mut last = false;

    loop {
        if next_start != 0 {
            if addr <= current_start {
                println!(
                    "warning: skipping image {} (0x{:x}) as no end signature found",
                    count - 1,
                    current_start
                );

                current_start = next_start;
                addr = next_start + 16;
                next_start = 0;
                continue;
            }

            let found_end = &buf[(addr - END.len())..addr] == END;
            let found_end_iml = &buf[(addr - END_IML.len())..addr] == END_IML;
            if found_end || found_end_iml {
                println!(
                    "found {}image {} at 0x{:x} to 0x{:x}",
                    if found_end_iml { "IML " } else { "" },
                    count - 1,
                    current_start,
                    addr - 1
                );

                std::fs::write(
                    out.join(format!(
                        "{}{}.sfi",
                        count - 1,
                        if found_end_iml { "-iml" } else { "" }
                    )),
                    &buf[current_start..addr],
                )
                .expect("failed to write output file");

                if last {
                    break;
                }

                current_start = next_start;
                addr = next_start + 16;
                next_start = 0;
                continue;
            }

            addr -= 1;
        } else {
            if &buf[addr..(addr + START.len())] == START {
                count += 1;

                if count == 1 {
                    current_start = addr;
                } else {
                    next_start = addr;
                    continue;
                }
            }

            // START is 16 bit aligned
            assert_eq!(addr % 16, 0);
            addr += 16;

            if addr + START.len() >= buf.len() {
                count += 1;
                next_start = buf.len();
                last = true;
            }
        }
    }
}
