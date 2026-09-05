#[doc = "Register `REG_PREFD3` reader"]
pub type R = crate::R<RegPrefd3Spec>;
#[doc = "Register `REG_PREFD3` writer"]
pub type W = crate::W<RegPrefd3Spec>;
#[doc = "Field `lsclk_prefd_3` reader - lsclk_prefd_3"]
pub type LsclkPrefd3R = crate::FieldReader<u16>;
#[doc = "Field `lsclk_prefd_3` writer - lsclk_prefd_3"]
pub type LsclkPrefd3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - lsclk_prefd_3"]
    #[inline(always)]
    pub fn lsclk_prefd_3(&self) -> LsclkPrefd3R {
        LsclkPrefd3R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - lsclk_prefd_3"]
    #[inline(always)]
    pub fn lsclk_prefd_3(&mut self) -> LsclkPrefd3W<'_, RegPrefd3Spec> {
        LsclkPrefd3W::new(self, 0)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPrefd3Spec;
impl crate::RegisterSpec for RegPrefd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_prefd3::R`](R) reader structure"]
impl crate::Readable for RegPrefd3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_prefd3::W`](W) writer structure"]
impl crate::Writable for RegPrefd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PREFD3 to value 0"]
impl crate::Resettable for RegPrefd3Spec {}
