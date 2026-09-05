#[doc = "Register `CR_RDFFTHRES` reader"]
pub type R = crate::R<CrRdffthresSpec>;
#[doc = "Register `CR_RDFFTHRES` writer"]
pub type W = crate::W<CrRdffthresSpec>;
#[doc = "Field `cr_rdffthres` reader - cr_rdffthres read/write control register"]
pub type CrRdffthresR = crate::FieldReader;
#[doc = "Field `cr_rdffthres` writer - cr_rdffthres read/write control register"]
pub type CrRdffthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cr_rdffthres read/write control register"]
    #[inline(always)]
    pub fn cr_rdffthres(&self) -> CrRdffthresR {
        CrRdffthresR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cr_rdffthres read/write control register"]
    #[inline(always)]
    pub fn cr_rdffthres(&mut self) -> CrRdffthresW<'_, CrRdffthresSpec> {
        CrRdffthresW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rdffthres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rdffthres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRdffthresSpec;
impl crate::RegisterSpec for CrRdffthresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_rdffthres::R`](R) reader structure"]
impl crate::Readable for CrRdffthresSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_rdffthres::W`](W) writer structure"]
impl crate::Writable for CrRdffthresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_RDFFTHRES to value 0"]
impl crate::Resettable for CrRdffthresSpec {}
