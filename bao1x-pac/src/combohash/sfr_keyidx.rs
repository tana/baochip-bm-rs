#[doc = "Register `SFR_KEYIDX` reader"]
pub type R = crate::R<SfrKeyidxSpec>;
#[doc = "Register `SFR_KEYIDX` writer"]
pub type W = crate::W<SfrKeyidxSpec>;
#[doc = "Field `sfr_keyidx` reader - sfr_keyidx read/write control register"]
pub type SfrKeyidxR = crate::FieldReader<u16>;
#[doc = "Field `sfr_keyidx` writer - sfr_keyidx read/write control register"]
pub type SfrKeyidxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sfr_keyidx read/write control register"]
    #[inline(always)]
    pub fn sfr_keyidx(&self) -> SfrKeyidxR {
        SfrKeyidxR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sfr_keyidx read/write control register"]
    #[inline(always)]
    pub fn sfr_keyidx(&mut self) -> SfrKeyidxW<'_, SfrKeyidxSpec> {
        SfrKeyidxW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L217 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L217>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_keyidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_keyidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrKeyidxSpec;
impl crate::RegisterSpec for SfrKeyidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_keyidx::R`](R) reader structure"]
impl crate::Readable for SfrKeyidxSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_keyidx::W`](W) writer structure"]
impl crate::Writable for SfrKeyidxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_KEYIDX to value 0"]
impl crate::Resettable for SfrKeyidxSpec {}
