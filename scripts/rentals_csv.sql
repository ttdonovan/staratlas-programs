COPY (
    SELECT
        rc.fleet,
        rc.rate AS fleet_rental_rate,
        rc.current_rental_state AS fleet_rental_state,
        f.faction,
        json_extract (f.ship_counts, '$.total') AS fleet_ship_counts,
        f.fleet_label,
        s.name AS ship_name,
        s.size_class AS ship_size,
        fs.amount AS ship_count,
        fs.idx AS ship_index,
        s.mint AS ship_mint,
        rc.owner_profile
    FROM
        db.rental_contract_states AS rc
        JOIN db.sage_fleets AS f on f.pubkey = rc.fleet
        JOIN db.sage_fleet_ships AS fs ON fs.pubkey = f.fleet_ships
        JOIN db.sage_ships AS s ON s.pubkey = fs.ship
) TO 'tmp/rentals.csv' (HEADER, DELIMITER ',');
