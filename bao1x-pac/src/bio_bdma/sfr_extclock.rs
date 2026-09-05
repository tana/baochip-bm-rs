#[doc = "Register `SFR_EXTCLOCK` reader"]
pub type R = crate::R<SfrExtclockSpec>;
#[doc = "Register `SFR_EXTCLOCK` writer"]
pub type W = crate::W<SfrExtclockSpec>;
#[doc = "Field `use_extclk` reader - use_extclk read/write control register"]
pub type UseExtclkR = crate::FieldReader;
#[doc = "Field `use_extclk` writer - use_extclk read/write control register"]
pub type UseExtclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `extclk_gpio_0` reader - extclk_gpio_0 read/write control register"]
pub type ExtclkGpio0R = crate::FieldReader;
#[doc = "Field `extclk_gpio_0` writer - extclk_gpio_0 read/write control register"]
pub type ExtclkGpio0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `extclk_gpio_1` reader - extclk_gpio_1 read/write control register"]
pub type ExtclkGpio1R = crate::FieldReader;
#[doc = "Field `extclk_gpio_1` writer - extclk_gpio_1 read/write control register"]
pub type ExtclkGpio1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `extclk_gpio_2` reader - extclk_gpio_2 read/write control register"]
pub type ExtclkGpio2R = crate::FieldReader;
#[doc = "Field `extclk_gpio_2` writer - extclk_gpio_2 read/write control register"]
pub type ExtclkGpio2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `extclk_gpio_3` reader - extclk_gpio_3 read/write control register"]
pub type ExtclkGpio3R = crate::FieldReader;
#[doc = "Field `extclk_gpio_3` writer - extclk_gpio_3 read/write control register"]
pub type ExtclkGpio3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - use_extclk read/write control register"]
    #[inline(always)]
    pub fn use_extclk(&self) -> UseExtclkR {
        UseExtclkR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - extclk_gpio_0 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_0(&self) -> ExtclkGpio0R {
        ExtclkGpio0R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - extclk_gpio_1 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_1(&self) -> ExtclkGpio1R {
        ExtclkGpio1R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - extclk_gpio_2 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_2(&self) -> ExtclkGpio2R {
        ExtclkGpio2R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - extclk_gpio_3 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_3(&self) -> ExtclkGpio3R {
        ExtclkGpio3R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - use_extclk read/write control register"]
    #[inline(always)]
    pub fn use_extclk(&mut self) -> UseExtclkW<'_, SfrExtclockSpec> {
        UseExtclkW::new(self, 0)
    }
    #[doc = "Bits 4:8 - extclk_gpio_0 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_0(&mut self) -> ExtclkGpio0W<'_, SfrExtclockSpec> {
        ExtclkGpio0W::new(self, 4)
    }
    #[doc = "Bits 9:13 - extclk_gpio_1 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_1(&mut self) -> ExtclkGpio1W<'_, SfrExtclockSpec> {
        ExtclkGpio1W::new(self, 9)
    }
    #[doc = "Bits 14:18 - extclk_gpio_2 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_2(&mut self) -> ExtclkGpio2W<'_, SfrExtclockSpec> {
        ExtclkGpio2W::new(self, 14)
    }
    #[doc = "Bits 19:23 - extclk_gpio_3 read/write control register"]
    #[inline(always)]
    pub fn extclk_gpio_3(&mut self) -> ExtclkGpio3W<'_, SfrExtclockSpec> {
        ExtclkGpio3W::new(self, 19)
    }
}
#[doc = "See `bio_bdma.sv#L508 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L508>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_extclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_extclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrExtclockSpec;
impl crate::RegisterSpec for SfrExtclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_extclock::R`](R) reader structure"]
impl crate::Readable for SfrExtclockSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_extclock::W`](W) writer structure"]
impl crate::Writable for SfrExtclockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_EXTCLOCK to value 0"]
impl crate::Resettable for SfrExtclockSpec {}
