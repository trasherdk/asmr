use anyhow::Context;

use crate::coins::*;

pub async fn test_funded_address_created_lock(
  mut host: AnyScriptedHost,
  mut hosts_verifier: AnyUnscriptedVerifier,
  mut client: AnyUnscriptedClient,
  mut clients_verifier: AnyScriptedVerifier
) -> anyhow::Result<bool> {
  let host_keys = host.generate_keys(&mut hosts_verifier);
  let client_keys = client.generate_keys(&mut clients_verifier);
  host.verify_keys(&client_keys, &mut hosts_verifier).context("Couldn't verify client DlEq proof")?;
  client.verify_keys(&host_keys, &mut clients_verifier).context("Couldn't verify host DlEq proof")?;

  let _ = host.generate_deposit_address();
  host.send_from_node().await?;
  host.advance_consensus().await?;

  let _ = host.create_lock_and_prepare_refund().await.context("Couldn't create the BTC lock")?;
  host.refund(hosts_verifier).await?;
  Ok(true)
}
