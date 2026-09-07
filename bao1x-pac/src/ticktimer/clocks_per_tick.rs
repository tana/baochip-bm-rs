#[doc = "Register `CLOCKS_PER_TICK` reader"]
pub type R = crate::R<ClocksPerTickSpec>;
#[doc = "Register `CLOCKS_PER_TICK` writer"]
pub type W = crate::W<ClocksPerTickSpec>;
#[doc = "Field `clocks_per_tick` reader - "]
pub type ClocksPerTickR = crate::FieldReader<u32>;
#[doc = "Field `clocks_per_tick` writer - "]
pub type ClocksPerTickW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clocks_per_tick(&self) -> ClocksPerTickR {
        ClocksPerTickR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clocks_per_tick(&mut self) -> ClocksPerTickW<'_, ClocksPerTickSpec> {
        ClocksPerTickW::new(self, 0)
    }
}
#[doc = "Clocks per tick, defaults to 800000\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_per_tick::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_per_tick::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClocksPerTickSpec;
impl crate::RegisterSpec for ClocksPerTickSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clocks_per_tick::R`](R) reader structure"]
impl crate::Readable for ClocksPerTickSpec {}
#[doc = "`write(|w| ..)` method takes [`clocks_per_tick::W`](W) writer structure"]
impl crate::Writable for ClocksPerTickSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKS_PER_TICK to value 0x000c_3500"]
impl crate::Resettable for ClocksPerTickSpec {
    const RESET_VALUE: u32 = 0x000c_3500;
}
