#[doc = "Register `CR_CLK1HZFD` reader"]
pub type R = crate::R<CrClk1hzfdSpec>;
#[doc = "Register `CR_CLK1HZFD` writer"]
pub type W = crate::W<CrClk1hzfdSpec>;
#[doc = "Field `cr_clk1hzfd` reader - cr_clk1hzfd read/write control register"]
pub type CrClk1hzfdR = crate::FieldReader<u16>;
#[doc = "Field `cr_clk1hzfd` writer - cr_clk1hzfd read/write control register"]
pub type CrClk1hzfdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - cr_clk1hzfd read/write control register"]
    #[inline(always)]
    pub fn cr_clk1hzfd(&self) -> CrClk1hzfdR {
        CrClk1hzfdR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - cr_clk1hzfd read/write control register"]
    #[inline(always)]
    pub fn cr_clk1hzfd(&mut self) -> CrClk1hzfdW<'_, CrClk1hzfdSpec> {
        CrClk1hzfdW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L368 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L368>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_clk1hzfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_clk1hzfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrClk1hzfdSpec;
impl crate::RegisterSpec for CrClk1hzfdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_clk1hzfd::R`](R) reader structure"]
impl crate::Readable for CrClk1hzfdSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_clk1hzfd::W`](W) writer structure"]
impl crate::Writable for CrClk1hzfdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_CLK1HZFD to value 0"]
impl crate::Resettable for CrClk1hzfdSpec {}
