#[doc = "Register `REG_TIM2_CMD` reader"]
pub type R = crate::R<RegTim2CmdSpec>;
#[doc = "Register `REG_TIM2_CMD` writer"]
pub type W = crate::W<RegTim2CmdSpec>;
#[doc = "Field `r_timer2_start` reader - r_timer2_start"]
pub type RTimer2StartR = crate::BitReader;
#[doc = "Field `r_timer2_start` writer - r_timer2_start"]
pub type RTimer2StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_stop` reader - r_timer2_stop"]
pub type RTimer2StopR = crate::BitReader;
#[doc = "Field `r_timer2_stop` writer - r_timer2_stop"]
pub type RTimer2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_update` reader - r_timer2_update"]
pub type RTimer2UpdateR = crate::BitReader;
#[doc = "Field `r_timer2_update` writer - r_timer2_update"]
pub type RTimer2UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_rst` reader - r_timer2_rst"]
pub type RTimer2RstR = crate::BitReader;
#[doc = "Field `r_timer2_rst` writer - r_timer2_rst"]
pub type RTimer2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer2_arm` reader - r_timer2_arm"]
pub type RTimer2ArmR = crate::BitReader;
#[doc = "Field `r_timer2_arm` writer - r_timer2_arm"]
pub type RTimer2ArmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_timer2_start"]
    #[inline(always)]
    pub fn r_timer2_start(&self) -> RTimer2StartR {
        RTimer2StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_timer2_stop"]
    #[inline(always)]
    pub fn r_timer2_stop(&self) -> RTimer2StopR {
        RTimer2StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - r_timer2_update"]
    #[inline(always)]
    pub fn r_timer2_update(&self) -> RTimer2UpdateR {
        RTimer2UpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - r_timer2_rst"]
    #[inline(always)]
    pub fn r_timer2_rst(&self) -> RTimer2RstR {
        RTimer2RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_timer2_arm"]
    #[inline(always)]
    pub fn r_timer2_arm(&self) -> RTimer2ArmR {
        RTimer2ArmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_timer2_start"]
    #[inline(always)]
    pub fn r_timer2_start(&mut self) -> RTimer2StartW<'_, RegTim2CmdSpec> {
        RTimer2StartW::new(self, 0)
    }
    #[doc = "Bit 1 - r_timer2_stop"]
    #[inline(always)]
    pub fn r_timer2_stop(&mut self) -> RTimer2StopW<'_, RegTim2CmdSpec> {
        RTimer2StopW::new(self, 1)
    }
    #[doc = "Bit 2 - r_timer2_update"]
    #[inline(always)]
    pub fn r_timer2_update(&mut self) -> RTimer2UpdateW<'_, RegTim2CmdSpec> {
        RTimer2UpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - r_timer2_rst"]
    #[inline(always)]
    pub fn r_timer2_rst(&mut self) -> RTimer2RstW<'_, RegTim2CmdSpec> {
        RTimer2RstW::new(self, 3)
    }
    #[doc = "Bit 4 - r_timer2_arm"]
    #[inline(always)]
    pub fn r_timer2_arm(&mut self) -> RTimer2ArmW<'_, RegTim2CmdSpec> {
        RTimer2ArmW::new(self, 4)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim2CmdSpec;
impl crate::RegisterSpec for RegTim2CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim2_cmd::R`](R) reader structure"]
impl crate::Readable for RegTim2CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim2_cmd::W`](W) writer structure"]
impl crate::Writable for RegTim2CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM2_CMD to value 0"]
impl crate::Resettable for RegTim2CmdSpec {}
