#[doc = "Register `REG_PREFD2` reader"]
pub type R = crate::R<RegPrefd2Spec>;
#[doc = "Register `REG_PREFD2` writer"]
pub type W = crate::W<RegPrefd2Spec>;
#[doc = "Field `lsclk_prefd_2` reader - lsclk_prefd_2"]
pub type LsclkPrefd2R = crate::FieldReader<u16>;
#[doc = "Field `lsclk_prefd_2` writer - lsclk_prefd_2"]
pub type LsclkPrefd2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - lsclk_prefd_2"]
    #[inline(always)]
    pub fn lsclk_prefd_2(&self) -> LsclkPrefd2R {
        LsclkPrefd2R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - lsclk_prefd_2"]
    #[inline(always)]
    pub fn lsclk_prefd_2(&mut self) -> LsclkPrefd2W<'_, RegPrefd2Spec> {
        LsclkPrefd2W::new(self, 0)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPrefd2Spec;
impl crate::RegisterSpec for RegPrefd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_prefd2::R`](R) reader structure"]
impl crate::Readable for RegPrefd2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_prefd2::W`](W) writer structure"]
impl crate::Writable for RegPrefd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PREFD2 to value 0"]
impl crate::Resettable for RegPrefd2Spec {}
