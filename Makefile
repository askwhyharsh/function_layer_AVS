# Start anvil and deploy contracts
up-anvil-and-contracts:
	@echo "Cleaning up any existing Anvil process..."
	@-pkill -f "anvil" || true
	@echo "Starting Anvil..."
	@anvil --port 3001 --fork-url https://eth.drpc.org > /dev/null 2>&1 & echo $$! > anvil.pid
	@echo "Waiting for Anvil to start..."
	@sleep 5
	@echo "Deploying contracts..."
	@cd contracts && forge script DeployServiceManager --rpc-url http://localhost:3001 --broadcast
	@kill `cat anvil.pid` && rm anvil.pid
	@echo "Deployment complete"

up-contracts:
	@echo "Deploying contracts..."
	@cd contracts && forge script DeployServiceManager --rpc-url http://localhost:3001 --broadcast
	@echo "Deployment complete"

up-anvil:
	@echo "Cleaning up any existing Anvil process..."
	@-pkill -f "anvil" || true
	@echo "Starting Anvil..."
	@anvil --port 3001 --fork-url https://eth.drpc.org & echo $$! > anvil.pid
	@echo "Waiting for Anvil to start..."
	@sleep 5
	@echo "Anvil started"

test-flow-of