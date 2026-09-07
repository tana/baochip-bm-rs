#[doc = "Register `EV_EDGE_TRIGGERED` reader"]
pub type R = crate::R<EvEdgeTriggeredSpec>;
#[doc = "Register `EV_EDGE_TRIGGERED` writer"]
pub type W = crate::W<EvEdgeTriggeredSpec>;
#[doc = "Field `use_edge` reader - None"]
pub type UseEdgeR = crate::FieldReader<u16>;
#[doc = "Field `use_edge` writer - None"]
pub type UseEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn use_edge(&self) -> UseEdgeR {
        UseEdgeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn use_edge(&mut self) -> UseEdgeW<'_, EvEdgeTriggeredSpec> {
        UseEdgeW::new(self, 0)
    }
}
#[doc = "If a bit is set to 1, then the hardware trigger is edge-triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_edge_triggered::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_edge_triggered::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEdgeTriggeredSpec;
impl crate::RegisterSpec for EvEdgeTriggeredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_edge_triggered::R`](R) reader structure"]
impl crate::Readable for EvEdgeTriggeredSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_edge_triggered::W`](W) writer structure"]
impl crate::Writable for EvEdgeTriggeredSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_EDGE_TRIGGERED to value 0"]
impl crate::Resettable for EvEdgeTriggeredSpec {}
