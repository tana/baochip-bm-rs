#[doc = "Register `CR_WKUPMASK` reader"]
pub type R = crate::R<CrWkupmaskSpec>;
#[doc = "Register `CR_WKUPMASK` writer"]
pub type W = crate::W<CrWkupmaskSpec>;
#[doc = "Field `inten` reader - inten read/write control register"]
pub type IntenR = crate::FieldReader;
#[doc = "Field `inten` writer - inten read/write control register"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wkupmask` reader - wkupmask read/write control register"]
pub type WkupmaskR = crate::FieldReader<u16>;
#[doc = "Field `wkupmask` writer - wkupmask read/write control register"]
pub type WkupmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - inten read/write control register"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - wkupmask read/write control register"]
    #[inline(always)]
    pub fn wkupmask(&self) -> WkupmaskR {
        WkupmaskR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - inten read/write control register"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, CrWkupmaskSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bits 8:17 - wkupmask read/write control register"]
    #[inline(always)]
    pub fn wkupmask(&mut self) -> WkupmaskW<'_, CrWkupmaskSpec> {
        WkupmaskW::new(self, 8)
    }
}
#[doc = "See `ao_sysctrl.sv#L369 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L369>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_wkupmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_wkupmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrWkupmaskSpec;
impl crate::RegisterSpec for CrWkupmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_wkupmask::R`](R) reader structure"]
impl crate::Readable for CrWkupmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_wkupmask::W`](W) writer structure"]
impl crate::Writable for CrWkupmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_WKUPMASK to value 0"]
impl crate::Resettable for CrWkupmaskSpec {}
