
pub struct FPRC {
    pub ioe: bool,
    pub dze: bool,
    pub ofe: bool,
    pub ufe: bool,
    pub ixe: bool,
    pub ide: bool,
    pub rmode0: bool,
    pub rmode1: bool,
    pub fz: bool,
    pub dn: bool,
    pub ahp: bool,
}

impl FPRC {
    pub fn new() -> FPRC {
        FPRC {
            ioe: false,
            dze: false,
            ofe: false,
            ufe: false,
            ixe: false,
            ide: false,
            rmode0: false,
            rmode1: false,
            fz: false,
            dn: false,
            ahp: false,
        }
    }

    pub fn get_fpcr(&self) -> u32 {
        let mut fpcr = 0u32;
        if self.ioe {
            fpcr |= 1 << 8;
        }
        if self.dze {
            fpcr |= 1 << 9;
        }
        if self.ofe {
            fpcr |= 1 << 10;
        }
        if self.ufe {
            fpcr |= 1 << 11;
        }
        if self.ixe {
            fpcr |= 1 << 12;
        }
        if self.ide {
            fpcr |= 1 << 15;
        }
        if self.rmode0 {
            fpcr |= 1 << 22;
        }
        if self.rmode1 {
            fpcr |= 1 << 23;
        }
        if self.fz {
            fpcr |= 1 << 24;
        }
        if self.dn {
            fpcr |= 1 << 25;
        }
        if self.ahp {
            fpcr |= 1 << 26;
        }
        fpcr
    }

    pub fn set_fpcr(&mut self, fpcr: u32) {
        self.ioe = (fpcr & (1 << 8)) != 0;
        self.dze = (fpcr & (1 << 9)) != 0;
        self.ofe = (fpcr & (1 << 10)) != 0;
        self.ufe = (fpcr & (1 << 11)) != 0;
        self.ixe = (fpcr & (1 << 12)) != 0;
        self.ide = (fpcr & (1 << 15)) != 0;
        self.rmode0 = (fpcr & (1 << 22)) != 0;
        self.rmode1 = (fpc = (1 << 23)) != 0;
        self.fz = (fpcr & (1 << 24)) != 0;
        self.dn = (fpcr & (1 << 25)) != 0;
        self.ahp = (fpcr & (1 << 26)) != 0;
    }
}
