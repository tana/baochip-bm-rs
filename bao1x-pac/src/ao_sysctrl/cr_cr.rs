#[doc = "Register `CR_CR` reader"]
pub type R = crate::R<CrCrSpec>;
#[doc = "Register `CR_CR` writer"]
pub type W = crate::W<CrCrSpec>;
#[doc = "Field `clk32kselreg` reader - clk32kselreg read/write control register"]
pub type Clk32kselregR = crate::BitReader;
#[doc = "Field `clk32kselreg` writer - clk32kselreg read/write control register"]
pub type Clk32kselregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdisoen` reader - pdisoen read/write control register"]
pub type PdisoenR = crate::BitReader;
#[doc = "Field `pdisoen` writer - pdisoen read/write control register"]
pub type PdisoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pclkicg` reader - pclkicg read/write control register"]
pub type PclkicgR = crate::BitReader;
#[doc = "Field `pclkicg` writer - pclkicg read/write control register"]
pub type PclkicgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clk32kselreg read/write control register"]
    #[inline(always)]
    pub fn clk32kselreg(&self) -> Clk32kselregR {
        Clk32kselregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pdisoen read/write control register"]
    #[inline(always)]
    pub fn pdisoen(&self) -> PdisoenR {
        PdisoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclkicg read/write control register"]
    #[inline(always)]
    pub fn pclkicg(&self) -> PclkicgR {
        PclkicgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk32kselreg read/write control register"]
    #[inline(always)]
    pub fn clk32kselreg(&mut self) -> Clk32kselregW<'_, CrCrSpec> {
        Clk32kselregW::new(self, 0)
    }
    #[doc = "Bit 1 - pdisoen read/write control register"]
    #[inline(always)]
    pub fn pdisoen(&mut self) -> PdisoenW<'_, CrCrSpec> {
        PdisoenW::new(self, 1)
    }
    #[doc = "Bit 2 - pclkicg read/write control register"]
    #[inline(always)]
    pub fn pclkicg(&mut self) -> PclkicgW<'_, CrCrSpec> {
        PclkicgW::new(self, 2)
    }
}
#[doc = "See `ao_sysctrl.sv#L367 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L367>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrCrSpec;
impl crate::RegisterSpec for CrCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_cr::R`](R) reader structure"]
impl crate::Readable for CrCrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_cr::W`](W) writer structure"]
impl crate::Writable for CrCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_CR to value 0"]
impl crate::Resettable for CrCrSpec {}
