#[doc = "Register `SFR_ICH_RPSTART` reader"]
pub type R = crate::R<SfrIchRpstartSpec>;
#[doc = "Register `SFR_ICH_RPSTART` writer"]
pub type W = crate::W<SfrIchRpstartSpec>;
#[doc = "Field `ichcr_rpstart` reader - ichcr_rpstart read/write control register"]
pub type IchcrRpstartR = crate::FieldReader<u16>;
#[doc = "Field `ichcr_rpstart` writer - ichcr_rpstart read/write control register"]
pub type IchcrRpstartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ichcr_rpstart read/write control register"]
    #[inline(always)]
    pub fn ichcr_rpstart(&self) -> IchcrRpstartR {
        IchcrRpstartR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ichcr_rpstart read/write control register"]
    #[inline(always)]
    pub fn ichcr_rpstart(&mut self) -> IchcrRpstartW<'_, SfrIchRpstartSpec> {
        IchcrRpstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L113>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_rpstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_rpstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIchRpstartSpec;
impl crate::RegisterSpec for SfrIchRpstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ich_rpstart::R`](R) reader structure"]
impl crate::Readable for SfrIchRpstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ich_rpstart::W`](W) writer structure"]
impl crate::Writable for SfrIchRpstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICH_RPSTART to value 0"]
impl crate::Resettable for SfrIchRpstartSpec {}
