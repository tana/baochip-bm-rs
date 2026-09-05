#[doc = "Register `REG_TIM1_CMD` reader"]
pub type R = crate::R<RegTim1CmdSpec>;
#[doc = "Register `REG_TIM1_CMD` writer"]
pub type W = crate::W<RegTim1CmdSpec>;
#[doc = "Field `r_timer1_start` reader - r_timer1_start"]
pub type RTimer1StartR = crate::BitReader;
#[doc = "Field `r_timer1_start` writer - r_timer1_start"]
pub type RTimer1StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_stop` reader - r_timer1_stop"]
pub type RTimer1StopR = crate::BitReader;
#[doc = "Field `r_timer1_stop` writer - r_timer1_stop"]
pub type RTimer1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_update` reader - r_timer1_update"]
pub type RTimer1UpdateR = crate::BitReader;
#[doc = "Field `r_timer1_update` writer - r_timer1_update"]
pub type RTimer1UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_rst` reader - r_timer1_rst"]
pub type RTimer1RstR = crate::BitReader;
#[doc = "Field `r_timer1_rst` writer - r_timer1_rst"]
pub type RTimer1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer1_arm` reader - r_timer1_arm"]
pub type RTimer1ArmR = crate::BitReader;
#[doc = "Field `r_timer1_arm` writer - r_timer1_arm"]
pub type RTimer1ArmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_timer1_start"]
    #[inline(always)]
    pub fn r_timer1_start(&self) -> RTimer1StartR {
        RTimer1StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_timer1_stop"]
    #[inline(always)]
    pub fn r_timer1_stop(&self) -> RTimer1StopR {
        RTimer1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - r_timer1_update"]
    #[inline(always)]
    pub fn r_timer1_update(&self) -> RTimer1UpdateR {
        RTimer1UpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - r_timer1_rst"]
    #[inline(always)]
    pub fn r_timer1_rst(&self) -> RTimer1RstR {
        RTimer1RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_timer1_arm"]
    #[inline(always)]
    pub fn r_timer1_arm(&self) -> RTimer1ArmR {
        RTimer1ArmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_timer1_start"]
    #[inline(always)]
    pub fn r_timer1_start(&mut self) -> RTimer1StartW<'_, RegTim1CmdSpec> {
        RTimer1StartW::new(self, 0)
    }
    #[doc = "Bit 1 - r_timer1_stop"]
    #[inline(always)]
    pub fn r_timer1_stop(&mut self) -> RTimer1StopW<'_, RegTim1CmdSpec> {
        RTimer1StopW::new(self, 1)
    }
    #[doc = "Bit 2 - r_timer1_update"]
    #[inline(always)]
    pub fn r_timer1_update(&mut self) -> RTimer1UpdateW<'_, RegTim1CmdSpec> {
        RTimer1UpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - r_timer1_rst"]
    #[inline(always)]
    pub fn r_timer1_rst(&mut self) -> RTimer1RstW<'_, RegTim1CmdSpec> {
        RTimer1RstW::new(self, 3)
    }
    #[doc = "Bit 4 - r_timer1_arm"]
    #[inline(always)]
    pub fn r_timer1_arm(&mut self) -> RTimer1ArmW<'_, RegTim1CmdSpec> {
        RTimer1ArmW::new(self, 4)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim1CmdSpec;
impl crate::RegisterSpec for RegTim1CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim1_cmd::R`](R) reader structure"]
impl crate::Readable for RegTim1CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim1_cmd::W`](W) writer structure"]
impl crate::Writable for RegTim1CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM1_CMD to value 0"]
impl crate::Resettable for RegTim1CmdSpec {}
