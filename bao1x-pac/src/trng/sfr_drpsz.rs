#[doc = "Register `SFR_DRPSZ` reader"]
pub type R = crate::R<SfrDrpszSpec>;
#[doc = "Register `SFR_DRPSZ` writer"]
pub type W = crate::W<SfrDrpszSpec>;
#[doc = "Field `sfr_drpsz` reader - sfr_drpsz read/write control register"]
pub type SfrDrpszR = crate::FieldReader<u32>;
#[doc = "Field `sfr_drpsz` writer - sfr_drpsz read/write control register"]
pub type SfrDrpszW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_drpsz read/write control register"]
    #[inline(always)]
    pub fn sfr_drpsz(&self) -> SfrDrpszR {
        SfrDrpszR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_drpsz read/write control register"]
    #[inline(always)]
    pub fn sfr_drpsz(&mut self) -> SfrDrpszW<'_, SfrDrpszSpec> {
        SfrDrpszW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L239 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L239>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drpsz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drpsz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDrpszSpec;
impl crate::RegisterSpec for SfrDrpszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_drpsz::R`](R) reader structure"]
impl crate::Readable for SfrDrpszSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_drpsz::W`](W) writer structure"]
impl crate::Writable for SfrDrpszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DRPSZ to value 0"]
impl crate::Resettable for SfrDrpszSpec {}
