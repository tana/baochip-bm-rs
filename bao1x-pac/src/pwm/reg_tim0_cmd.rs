#[doc = "Register `REG_TIM0_CMD` reader"]
pub type R = crate::R<RegTim0CmdSpec>;
#[doc = "Register `REG_TIM0_CMD` writer"]
pub type W = crate::W<RegTim0CmdSpec>;
#[doc = "Field `r_timer0_start` reader - r_timer0_start"]
pub type RTimer0StartR = crate::BitReader;
#[doc = "Field `r_timer0_start` writer - r_timer0_start"]
pub type RTimer0StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_stop` reader - r_timer0_stop"]
pub type RTimer0StopR = crate::BitReader;
#[doc = "Field `r_timer0_stop` writer - r_timer0_stop"]
pub type RTimer0StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_update` reader - r_timer0_update"]
pub type RTimer0UpdateR = crate::BitReader;
#[doc = "Field `r_timer0_update` writer - r_timer0_update"]
pub type RTimer0UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_rst` reader - r_timer0_rst"]
pub type RTimer0RstR = crate::BitReader;
#[doc = "Field `r_timer0_rst` writer - r_timer0_rst"]
pub type RTimer0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_timer0_arm` reader - r_timer0_arm"]
pub type RTimer0ArmR = crate::BitReader;
#[doc = "Field `r_timer0_arm` writer - r_timer0_arm"]
pub type RTimer0ArmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_timer0_start"]
    #[inline(always)]
    pub fn r_timer0_start(&self) -> RTimer0StartR {
        RTimer0StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_timer0_stop"]
    #[inline(always)]
    pub fn r_timer0_stop(&self) -> RTimer0StopR {
        RTimer0StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - r_timer0_update"]
    #[inline(always)]
    pub fn r_timer0_update(&self) -> RTimer0UpdateR {
        RTimer0UpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - r_timer0_rst"]
    #[inline(always)]
    pub fn r_timer0_rst(&self) -> RTimer0RstR {
        RTimer0RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_timer0_arm"]
    #[inline(always)]
    pub fn r_timer0_arm(&self) -> RTimer0ArmR {
        RTimer0ArmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_timer0_start"]
    #[inline(always)]
    pub fn r_timer0_start(&mut self) -> RTimer0StartW<'_, RegTim0CmdSpec> {
        RTimer0StartW::new(self, 0)
    }
    #[doc = "Bit 1 - r_timer0_stop"]
    #[inline(always)]
    pub fn r_timer0_stop(&mut self) -> RTimer0StopW<'_, RegTim0CmdSpec> {
        RTimer0StopW::new(self, 1)
    }
    #[doc = "Bit 2 - r_timer0_update"]
    #[inline(always)]
    pub fn r_timer0_update(&mut self) -> RTimer0UpdateW<'_, RegTim0CmdSpec> {
        RTimer0UpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - r_timer0_rst"]
    #[inline(always)]
    pub fn r_timer0_rst(&mut self) -> RTimer0RstW<'_, RegTim0CmdSpec> {
        RTimer0RstW::new(self, 3)
    }
    #[doc = "Bit 4 - r_timer0_arm"]
    #[inline(always)]
    pub fn r_timer0_arm(&mut self) -> RTimer0ArmW<'_, RegTim0CmdSpec> {
        RTimer0ArmW::new(self, 4)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTim0CmdSpec;
impl crate::RegisterSpec for RegTim0CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim0_cmd::R`](R) reader structure"]
impl crate::Readable for RegTim0CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tim0_cmd::W`](W) writer structure"]
impl crate::Writable for RegTim0CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TIM0_CMD to value 0"]
impl crate::Resettable for RegTim0CmdSpec {}
