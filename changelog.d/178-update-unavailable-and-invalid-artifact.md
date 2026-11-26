- Add two explicit update states. `UpdateAvailabilityProjection` gains
  `unavailableForTarget` (with `version`): the channel has a release but no
  artifact for this system. It renders as information with no action.
- Add the `malformedArtifact` rejection code, with `lastRejectionDetail` on
  `UpdateStatus` and `UpdateCenter`. It renders as the invalid-artifact state
  (warning tone, the installer detail as its body) and offers a new
  `downloadAgain` action, emitted as `onDownloadAgain` (label:
  `downloadAgainLabel`). Download again, not check again.
- Both additions are additive: existing hosts keep working and gain the states
  when the authority reports them.
