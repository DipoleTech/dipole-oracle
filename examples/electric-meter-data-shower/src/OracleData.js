import React, { useState, useEffect } from 'react';
import { List, Grid, Button, Input, Modal } from 'semantic-ui-react';

// Pre-built Substrate front-end utilities for connecting to a node
// and making a transaction.
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
// Polkadot-JS utilities for hashing data.
import { blake2AsHex } from '@polkadot/util-crypto';

export default function Main(props) {
  const { api, keyring } = useSubstrate();
	const { accountPair } = props;
	// The transaction submission status
	const [ status, setStatus ] = useState('');

	// The currently stored value

	const [ collectorDatasList, setCollectorDatasList ] = useState([]);
	const [ collectorDatasNumber, setCollectorDatasNumber ] = useState([]);
	const [ itemRenders, setItemRenders ] = useState(0);

	const keyringOptions = keyring.getPairs().map((account) => ({
		key: account.address,
		value: account.address,
		text: account.meta.name.toUpperCase(),
		icon: 'user',
	}));

	const getKeyringName = (addr) => {
		let name = '';
		keyringOptions.map((value) => {
			if (name === '' && value.key === addr) {
				name = value.text;
			}
		});
		if (name !== '') return name;
		return addr;
	};

	useEffect(
		() => {
			let unsubscribe;
			if (accountPair) {
				api.query.collector
					.collectorDatas( (arr) => {
						if (arr.isNone) {
							setCollectorDatasList([]);
						} else {
							let list = arr.toJSON();

							setCollectorDatasList(list.goods_oracle_data);
							setCollectorDatasNumber(list.goods_oracle_data.length);
						}
					})
					.then((unsub) => {
						unsubscribe = unsub;
					})
					.catch(console.error);
			}
			return () => unsubscribe && unsubscribe();
		},
		[ api.query.collector, accountPair],
	);

	useEffect(
		() => {
			getItemRenders(collectorDatasList);
		},
		[ accountPair, collectorDatasList],
	);

	const getItemRenders = async (collectorDatasList) => {
		let list = [];

		for (var i = 0; i < collectorDatasList.length; i++) {
			if((collectorDatasList.length - i) > 5){
				continue
			}
			let puclist = [];
			let puplist = [];
			let prclist = [];
			let prplist = [];
			let data = collectorDatasList[i];
			//public_consumer_volume
			if (data.public_consumer_volume.length >0){
				for (var j = 0; j < data.public_consumer_volume.length ; j++) {
					let pucv = data.public_consumer_volume[j];
					puclist.push(
						<List.Item key={j}>
							<br /><List.Content>
								<div style={{ fontSize: 16 }}>
								&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;type:{pucv.volume_type}, volume:{pucv.volume}KWh
								</div>
							<br /></List.Content>
						</List.Item>,
					);
				}
			}
			//public_producer_volume
			if (data.public_producer_volume.length >0){
				for (var j = 0; j < data.public_producer_volume.length ; j++) {
					let pupv = data.public_producer_volume[j];
					puplist.push(
						<List.Item key={j}>
							<br /><List.Content>
								<div style={{ fontSize: 16 }}>
								&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;type:{pupv.volume_type}, volume:{pupv.volume}KWh
								</div>
							<br /></List.Content>
						</List.Item>,
					);
				}
			}
			//private_consumer_volume
			if (data.private_consumer_volume.length >0){
				for (var j = 0; j < data.private_consumer_volume.length ; j++) {
					let prcv = data.private_consumer_volume[j];
					prclist.push(
						<List.Item key={j}>
							<br /><List.Content>
								<div style={{ fontSize: 16 }}>
								&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;type:{prcv.volume_type}, volume:{prcv.volume}KWh
								</div>
							<br /></List.Content>
						</List.Item>,
					);
				}

			}
			//private_producer_volume
			if (data.private_producer_volume.length >0){
				for (var j = 0; j < data.public_producer_volume.length ; j++) {
					let prpv = data.public_producer_volume[j];
					prplist.push(
						<List.Item key={j}>
							<List.Content>
								<div style={{ fontSize: 16 }}>
								&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;type:{prpv.volume_type}, volume:{prpv.volume}KWh
								</div>
							</List.Content>
						</List.Item>,
					);
				}

			}
      		list.push(
				<List.Item key={i}>
					<br /><List.Content
						style={{
							padding: 20,
							backgroundColor: '#fff',
							fontSize: 18,
							color: '#333',
						}}>
						<div style={{ fontSize: 18 }}>
							<br />No:{i}
							<br /><br />Owner:{data.owner}
							<br /><br />*StateGrid* Electricity Consumption Data:{ puclist}
							{/* <br /><br />**StateGrid-ElectricPower-ProduceData:{ puplist} */}
							<br /><br />*CleanEnergy* Electricity Consumption Data:{ prclist}
							{/* <br /><br />**CleanEnergy-ElectricPower-ProduceData:{ prplist} */}
						</div>
					<br /></List.Content>
				</List.Item>,
			);
		}
    	setItemRenders(list);
	};

	return (
		<Grid.Column
			style={{
				backgroundColor: '#eceefa',
				padding: 20,
			}}
		>
			<h1>Auto Electric Meter Data</h1>
			<List>{itemRenders}</List>
			<div style={{ overflowWrap: 'break-word' }}>{status}</div>
		</Grid.Column>
  );
}




