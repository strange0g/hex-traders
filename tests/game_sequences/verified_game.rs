//! One complete accepted stream plus ordinary save/resume verification at each command.
use hex_traders_engine::{
    Command, CommandEnvelope, Engine, PlayerId, Receipt, ReplayVerifier, Viewer,
};
use std::ops::{Deref, DerefMut};

pub(super) struct VerifiedGame {
    engine: Engine,
    replay: ReplayVerifier,
}
impl VerifiedGame {
    pub(super) fn new(engine: Engine) -> Self {
        let replay = ReplayVerifier::from_header(&engine.export_replay_header().unwrap()).unwrap();
        assert_eq!(
            replay.engine().export_snapshot().unwrap(),
            engine.export_snapshot().unwrap()
        );
        Self { engine, replay }
    }
    pub(super) fn act(&mut self, actor: u8, command: Command) -> Receipt {
        let before = self.engine.export_snapshot().unwrap();
        let mut resumed = Engine::from_snapshot(&before).unwrap();
        assert_eq!(resumed.export_snapshot().unwrap(), before);
        let view = self.engine.view(Viewer::Spectator).unwrap();
        for &seat in &view.seat_order {
            assert_eq!(
                self.engine.legal_actions(seat).unwrap(),
                resumed.legal_actions(seat).unwrap()
            );
        }
        let actor = PlayerId::try_from(actor).unwrap();
        assert!(self.engine.validate(actor, &command).is_ok());
        assert_eq!(
            self.engine.validate(actor, &command),
            resumed.validate(actor, &command)
        );
        assert_eq!(self.engine.export_snapshot().unwrap(), before);
        let envelope = CommandEnvelope {
            api_version: 1,
            actor,
            expected_revision: view.revision,
            command,
        };
        let receipt = self.engine.execute(envelope).unwrap();
        assert_eq!(
            (receipt.previous_revision, receipt.revision),
            (view.revision, view.revision + 1)
        );
        assert_eq!(resumed.execute(envelope).unwrap(), receipt);
        let replayed = self
            .replay
            .push(&receipt.to_replay_record().unwrap())
            .unwrap();
        assert_eq!(replayed, receipt);
        let after = self.engine.export_snapshot().unwrap();
        if !matches!(
            command,
            Command::RollDice | Command::MoveRobber { .. } | Command::ChooseRobberVictim { .. }
        ) {
            let before: serde_json::Value = serde_json::from_slice(&before).unwrap();
            let after: serde_json::Value = serde_json::from_slice(&after).unwrap();
            assert_eq!(before["random"], after["random"]);
        }
        assert_eq!(resumed.export_snapshot().unwrap(), after);
        assert_eq!(self.replay.engine().export_snapshot().unwrap(), after);
        assert_eq!(
            Engine::from_snapshot(&after)
                .unwrap()
                .export_snapshot()
                .unwrap(),
            after
        );
        for viewer in
            std::iter::once(Viewer::Spectator).chain(view.seat_order.into_iter().map(Viewer::Seat))
        {
            assert_eq!(
                self.engine.view(viewer).unwrap().to_json().unwrap(),
                resumed.view(viewer).unwrap().to_json().unwrap()
            );
            assert_eq!(
                self.engine.view(viewer).unwrap().to_json().unwrap(),
                self.replay
                    .engine()
                    .view(viewer)
                    .unwrap()
                    .to_json()
                    .unwrap()
            );
            assert_eq!(
                self.engine.project_receipt(&receipt, viewer).unwrap(),
                resumed.project_receipt(&receipt, viewer).unwrap()
            );
            assert_eq!(
                self.engine.project_receipt(&receipt, viewer).unwrap(),
                self.replay
                    .engine()
                    .project_receipt(&replayed, viewer)
                    .unwrap()
            );
        }
        assert_eq!(self.engine.export_snapshot().unwrap(), after);
        assert!(self.engine.execute(envelope).is_err());
        assert_eq!(self.engine.export_snapshot().unwrap(), after);
        receipt
    }
}
impl Deref for VerifiedGame {
    type Target = Engine;
    fn deref(&self) -> &Engine {
        &self.engine
    }
}
impl DerefMut for VerifiedGame {
    fn deref_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}
