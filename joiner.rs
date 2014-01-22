use std::os;
use std::io::File;

fn main() 
{
    let args: ~[~str] = os::args();
    if args.len() != 3 
    {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } 
    else 
    {
        let fname1 = args[1].clone();
        let fname2 = args[2].clone();

        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());

        let share1_file = File::open(&path1);
        let share2_file = File::open(&path2);

        match (share1_file) 
        {
            Some(mut f1) => 
            {
                match(share2_file)
                {
                    Some(mut f2) =>
                    {
                        let msg_bytes1: ~[u8] = f1.read_to_end();
                        let msg_bytes2: ~[u8] = f2.read_to_end();

                        let file = File::create(&Path::new("joined.file"));

                        match (file) 
                        {
                            Some(f) =>
                            {
                                join(msg_bytes1, msg_bytes2, f);
                            },
                            _ => fail!("Error opening output file!")
                        }
                    },

                    None => fail!("Error opening message file: {:s}", fname2)
                }
            } ,
            None => fail!("Error opening message file: {:s}", fname1)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] 
{
    let mut ret = ~[];
    for i in range(0, a.len()) 
    {
        ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes1: &[u8], msg_bytes2: &[u8], mut file: File) 
{
    let decoded_bytes = xor(msg_bytes1, msg_bytes2);

    file.write(decoded_bytes);
}