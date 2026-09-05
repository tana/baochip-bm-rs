#[doc = "Register `SFR_DBG2` reader"]
pub type R = crate::R<SfrDbg2Spec>;
#[doc = "Register `SFR_DBG2` writer"]
pub type W = crate::W<SfrDbg2Spec>;
#[doc = "Field `dbg_pc` reader - dbg_pc read only status register"]
pub type DbgPcR = crate::BitReader;
#[doc = "Field `dbg_pc` writer - dbg_pc read only status register"]
pub type DbgPcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trap` reader - trap read only status register"]
pub type TrapR = crate::BitReader;
#[doc = "Field `trap` writer - trap read only status register"]
pub type TrapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - dbg_pc read only status register"]
    #[inline(always)]
    pub fn dbg_pc(&self) -> DbgPcR {
        DbgPcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - trap read only status register"]
    #[inline(always)]
    pub fn trap(&self) -> TrapR {
        TrapR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dbg_pc read only status register"]
    #[inline(always)]
    pub fn dbg_pc(&mut self) -> DbgPcW<'_, SfrDbg2Spec> {
        DbgPcW::new(self, 0)
    }
    #[doc = "Bit 1 - trap read only status register"]
    #[inline(always)]
    pub fn trap(&mut self) -> TrapW<'_, SfrDbg2Spec> {
        TrapW::new(self, 1)
    }
}
#[doc = "See `bio_bdma.sv#L531 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L531>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDbg2Spec;
impl crate::RegisterSpec for SfrDbg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dbg2::R`](R) reader structure"]
impl crate::Readable for SfrDbg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dbg2::W`](W) writer structure"]
impl crate::Writable for SfrDbg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DBG2 to value 0"]
impl crate::Resettable for SfrDbg2Spec {}
