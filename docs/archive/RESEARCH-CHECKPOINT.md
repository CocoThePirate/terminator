# Research checkpoint

Resume design collaboratively, one decision at a time. Do not advance implementation before discussing the design.

Agreed: consequence prediction is a separate model. It reads shared world state plus proposed controls and predicts outcomes with uncertainty, including timing and latency uncertainty.

Intended design: separately trainable skill modules encode learned behaviors in weights. Shared world state provides context, and skills influence a controller that produces KBM controls. They are not fixed input sequences.

Open decision: should each skill contribute a learned representation that the controller combines with other skill representations into one KBM output? This was proposed but not yet confirmed.

Research phase: investigate representation interfaces, independent skill training, compatibility between checkpoints, and how to measure contributions when skills are combined.
