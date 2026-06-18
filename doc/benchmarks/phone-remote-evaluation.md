# Phone Remote Evaluation

## Purpose

Track whether a candidate YOLO model improves the webcam failure mode where a phone is
classified as `remote`.

## Fixed Evaluation Cases

Capture or keep at least five images for each case:

- `phone_white_back_close`: white phone back, close range, in hand.
- `phone_screen_on_close`: phone screen on, close range, in hand.
- `phone_half_occluded`: phone partly hidden by fingers.
- `remote_close`: real remote, close range.
- `empty_hand`: hand visible without object.
- `rectangular_negative`: mouse, charger, box, or other rectangular non-phone object.

## Acceptance Gates

- `phone` recall at confidence `0.25` is at least `90%` on local phone captures.
- `remote` false positives on local phone captures are below `5%`.
- Empty-frame and empty-hand false positives above confidence `0.40` do not occur during a
  two-minute webcam run.
- Dashboard overlay labels stay stable enough for visual inspection: no more than one label
  flip per ten-second analysis window on the same held object.

## Run Record

For each candidate model, record:

- model id
- artifact URI
- training dataset version id
- confidence threshold
- phone recall
- remote false-positive rate on phone captures
- empty/negative false-positive count
- qualitative dashboard notes
