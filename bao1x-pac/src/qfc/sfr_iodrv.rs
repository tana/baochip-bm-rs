#[doc = "Register `SFR_IODRV` reader"]
pub type R = crate::R<SfrIodrvSpec>;
#[doc = "Register `SFR_IODRV` writer"]
pub type W = crate::W<SfrIodrvSpec>;
#[doc = "Field `paddrvsel` reader - paddrvsel read/write control register"]
pub type PaddrvselR = crate::FieldReader<u16>;
#[doc = "Field `paddrvsel` writer - paddrvsel read/write control register"]
pub type PaddrvselW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - paddrvsel read/write control register"]
    #[inline(always)]
    pub fn paddrvsel(&self) -> PaddrvselR {
        PaddrvselR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - paddrvsel read/write control register"]
    #[inline(always)]
    pub fn paddrvsel(&mut self) -> PaddrvselW<'_, SfrIodrvSpec> {
        PaddrvselW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L191 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L191>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iodrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iodrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIodrvSpec;
impl crate::RegisterSpec for SfrIodrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_iodrv::R`](R) reader structure"]
impl crate::Readable for SfrIodrvSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_iodrv::W`](W) writer structure"]
impl crate::Writable for SfrIodrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IODRV to value 0"]
impl crate::Resettable for SfrIodrvSpec {}
