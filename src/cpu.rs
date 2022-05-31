use rand::Rng;
pub struct Cpu {
    pub PC: usize,
    data: Vec<u8>,
    pub rom: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            PC: 0,
            data: vec![0; 4096],
            rom: vec![0; 4096],
        }
    }

    pub fn step(&mut self) {
        self.PC += 1;

        let mut rng = rand::thread_rng();
        //update some values in vram
        for _i in 0..10 {
            let change_index = rng.gen_range(0..4096);
            let value = rng.gen_range(0..u8::MAX);

            self.data[change_index] = value;
        }
        //update some values in rom
        for _j in 0..10 {
            let change_index = rng.gen_range(0..4096);
            let value = rng.gen_range(0..u8::MAX);

            self.rom[change_index] = value;
        }
    }

    pub fn get_data(&mut self) -> &Vec<u8> {
        return &self.data;
    }
}
