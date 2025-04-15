SELECT
    rc.fleet,
    rc.rate AS rental_rate,
    rc.current_rental_state AS rental_state,
    fs.idx AS fleet_ships_index,
    fs.amount AS fleet_ships_amount,
    json_extract (f.ship_counts, '$.total') AS ship_counts_total,
    f.faction,
    f.fleet_label,
    s.name AS ship_name,
    s.mint AS ship_mint,
    rc.owner_profile
FROM
    db.rental_contract_states AS rc
    JOIN db.sage_fleets AS f on f.pubkey = rc.fleet
    JOIN db.sage_fleet_ships AS fs ON fs.pubkey = f.fleet_ships
    JOIN db.sage_ships AS s ON s.pubkey = fs.ship;
