#[doc = "Register `CR_REG_DSR` reader"]
pub type R = crate::R<CrRegDsrSpec>;
#[doc = "Register `CR_REG_DSR` writer"]
pub type W = crate::W<CrRegDsrSpec>;
#[doc = "Field `cr_reg_dsr` reader - cr_reg_dsr read/write control register"]
pub type CrRegDsrR = crate::FieldReader<u16>;
#[doc = "Field `cr_reg_dsr` writer - cr_reg_dsr read/write control register"]
pub type CrRegDsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_reg_dsr read/write control register"]
    #[inline(always)]
    pub fn cr_reg_dsr(&self) -> CrRegDsrR {
        CrRegDsrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_reg_dsr read/write control register"]
    #[inline(always)]
    pub fn cr_reg_dsr(&mut self) -> CrRegDsrW<'_, CrRegDsrSpec> {
        CrRegDsrW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L135 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L135>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_dsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_dsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegDsrSpec;
impl crate::RegisterSpec for CrRegDsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_dsr::R`](R) reader structure"]
impl crate::Readable for CrRegDsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_dsr::W`](W) writer structure"]
impl crate::Writable for CrRegDsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_DSR to value 0"]
impl crate::Resettable for CrRegDsrSpec {}
