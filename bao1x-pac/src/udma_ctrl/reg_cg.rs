#[doc = "Register `REG_CG` reader"]
pub type R = crate::R<RegCgSpec>;
#[doc = "Register `REG_CG` writer"]
pub type W = crate::W<RegCgSpec>;
#[doc = "Field `r_cg` reader - r_cg"]
pub type RCgR = crate::FieldReader;
#[doc = "Field `r_cg` writer - r_cg"]
pub type RCgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - r_cg"]
    #[inline(always)]
    pub fn r_cg(&self) -> RCgR {
        RCgR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - r_cg"]
    #[inline(always)]
    pub fn r_cg(&mut self) -> RCgW<'_, RegCgSpec> {
        RCgW::new(self, 0)
    }
}
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCgSpec;
impl crate::RegisterSpec for RegCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cg::R`](R) reader structure"]
impl crate::Readable for RegCgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cg::W`](W) writer structure"]
impl crate::Writable for RegCgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CG to value 0"]
impl crate::Resettable for RegCgSpec {}
