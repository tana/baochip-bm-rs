#[doc = "Register `REG_TIM3_CMD` reader"]
pub type R = crate::R<RegTim3CmdSpec>;
#[doc = "Register `REG_TIM3_CMD` writer"]
pub type W = crate::W<RegTim3CmdSpec>;
#[doc = "Field `r_timer3_start` reader - r_timer3_start"]
pub type RTimer3StartR = crate::BitReader;
#[doc = "Field `r_timer3_start` writer - r_timer3_start"]
pub type RTimer3StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_stop` reader - r_timer3_stop"]
pub type RTimer3StopR = crate::BitReader;
#[doc = "Field `r_timer3_stop` writer - r_timer3_stop"]
pub type RTimer3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_update` reader - r_timer3_update"]
pub type RTimer3UpdateR = crate::BitReader;
#[doc = "Field `r_timer3_update` writer - r_timer3_update"]
pub type RTimer3UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_rst` reader - r_timer3_rst"]
pub type RTimer3RstR = crate::BitReader;
#[doc = "Field `r_timer3_rst` writer - r_timer3_rst"]
pub type RTimer3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer3_arm` reader - r_timer3_arm"]
pub type RTimer3ArmR = crate::BitReader;
#[doc = "Field `r_timer3_arm` writer - r_timer3_arm"]
pub type RTimer3ArmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_timer3_start"]
    #[inline(always)]
    pub fn r_timer3_start(&self) -> RTimer3StartR {
        RTimer3StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_timer3_stop"]
    #[inline(always)]
    pub fn r_timer3_stop(&self) -> RTimer3StopR {
        RTimer3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - r_timer3_update"]
    #[inline(always)]
    pub fn r_timer3_update(&self) -> RTimer3UpdateR {
        RTimer3UpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - r_timer3_rst"]
    #[inline(always)]
    pub fn r_timer3_rst(&self) -> RTimer3RstR {
        RTimer3RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_timer3_arm"]
    #[inline(always)]
    pub fn r_timer3_arm(&self) -> RTimer3ArmR {
        RTimer3ArmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_timer3_start"]
    #[inline(always)]
    pub fn r_timer3_start(&mut self) -> RTimer3StartW<'_, RegTim3CmdSpec> {
        RTimer3StartW::new(self, 0)
    }
    #[doc = "Bit 1 - r_timer3_stop"]
    #[inline(always)]
    pub fn r_timer3_stop(&mut self) -> RTimer3StopW<'_, RegTim3CmdSpec> {
        RTimer3StopW::new(self, 1)
    }
    #[doc = "Bit 2 - r_timer3_update"]
    #[inline(always)]
    pub fn r_timer3_update(&mut self) -> RTimer3UpdateW<'_, RegTim3CmdSpec> {
        RTimer3UpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - r_timer3_rst"]
    #[inline(always)]
    pub fn r_timer3_rst(&mut self) -> RTimer3RstW<'_, RegTim3CmdSpec> {
        RTimer3RstW::new(self, 3)
    }
    #[doc = "Bit 4 - r_timer3_arm"]
    #[inline(always)]
    pub fn r_timer3_arm(&mut self) -> RTimer3ArmW<'_, RegTim3CmdSpec> {
        RTimer3ArmW::new(self, 4)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim3CmdSpec;
impl crate::RegisterSpec for RegTim3CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim3_cmd::R`](R) reader structure"]
impl crate::Readable for RegTim3CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim3_cmd::W`](W) writer structure"]
impl crate::Writable for RegTim3CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM3_CMD to value 0"]
impl crate::Resettable for RegTim3CmdSpec {}
