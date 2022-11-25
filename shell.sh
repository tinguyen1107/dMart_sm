#!/bin/bash
# set -e

attrib=1
color=35

# Define function
Help ()
{
   # Display Help
   echo "Usage:"
   echo "   Syntax:"
   echo $'\t'"scriptTemplate [flag <action: string>]"
   echo "\n   Flag:"
   echo $'\t'"-m | --dmart        dMart contract."
   echo $'\t'"-n | --nft          NFT contract."
   echo "\n   Action:"
   echo $'\t'"b | build           Build specific contract."
   echo $'\t'"o | override        Deploy specific contract with old contract id."
   echo $'\t'"n | new             Deploy specific contract with new contract id."
   echo
   exit
}

Highlight_print_line ()
{
  printf %b "\033[$attrib;${color}m|| => $@\033[m\n"
}

build=0
deploy=0
new=0

Handle_received_value () 
{
  local val=$1
  build=0
  deploy=0
  new=0
 
  case "$val" in
    b | build)
      build=1
      ;;
    o | override)
      build=1
      deploy=1
      ;;
    n | new)
      new=1
      build=1
      deploy=1
      ;;
  esac
}

# Init variable
OWNER_ACCOUNT_ID=nttin.testnet

NEW_DMART=0
NEW_NFT=0

BUILD_DMART=0
BUILD_NFT=0

DEPLOY_DMART=0
DEPLOY_NFT=0

# rep.run contract
DMART_CTX_ID=
# nft contract
NFT_CTX_ID=

#################################################################################

###  GET PARAMS               ##########            ########         ############

#################################################################################

while getopts :c-:a-:n-: flag
do
  case "${flag}" in
    -)
      case "${OPTARG}" in
        dmart) 
          val="${!OPTIND}"
          Handle_received_value $val
          NEW_DMART=$new
          BUILD_DMART=$build
          DEPLOY_DMART=$deploy
          ;;
        nft)
          val="${!OPTIND}"
          Handle_received_value $val
          NEW_NFT=$new
          BUILD_NFT=$build
          DEPLOY_NFT=$deploy
          ;;
        *)
          Help 
          exit
          ;;
      esac
      ;;
    m) 
      val="${!OPTIND}"
      Handle_received_value $val
      NEW_DMART=$new
      BUILD_MART=$build
      DEPLOY_MART=$deploy
      ;;
    n)
      val="${!OPTIND}"
      Handle_received_value $val
      NEW_NFT=$new
      BUILD_NFT=$build
      DEPLOY_NFT=$deploy
      ;;
    *)
      Help 
      exit
      ;;
  esac
  shift $(($optind + 1))
done

if [ $OPTIND -eq 1 ]; then
  Help
  exit
fi

#################################################################################

###  HANDLE                   ##########            ########         ############

#################################################################################

##################################################
## Handle contract dmart        ##################
##################################################
cd "dmart"
# Check new
if [ $NEW_DMART -eq 1 ]
then
  rm -rf neardev
fi

# Check build
if [ $BUILD_DMART -eq 1 ]
then
  echo "Building dmart..."
  RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
  mkdir -p res
  cp target/wasm32-unknown-unknown/release/*.wasm res/contract.wasm
fi

# Check deploy
if [ $DEPLOY_DMART -eq 1 ]
then
  wait
  near dev-deploy res/contract.wasm
fi

# Get ctx id
wait
DMART_CTX_ID=$(<neardev/dev-account)
cd ".."

##################################################
## Handle contract NFT    ########################
##################################################
cd "NFT"
# Check new
if [ $NEW_NFT -eq 1 ]
then
  rm -rf neardev
fi

# Check build
if [ $BUILD_NFT -eq 1 ]
then
  echo "Building nft..."
  sh ./scripts/build.sh
fi

# Check deploy
if [ $DEPLOY_NFT -eq 1 ]
then
  wait
  near dev-deploy --wasmFile ./res/non_fungible_token.wasm
fi

# Get ctx id
wait
NFT_CTX_ID=$(<neardev/dev-account)
cd ".."


#################################################################################

###  SET UP                   ##########            ########         ############

#################################################################################

#############################
# Set up NFT        ###   ###
#############################
if [ $NEW_NFT -eq 1 ]
then
  near call $NFT_CTX_ID new_default_meta '{"owner_id": "'$OWNER_ACCOUNT_ID'"}' --accountId $OWNER_ACCOUNT_ID
  near call $NFT_CTX_ID set_role '{"account_id": "'$DMART_CTX_ID'"}' --accountId $OWNER_ACCOUNT_ID
  
  #############################
  # Set up dMart      ###   ###
  #############################
  if [ $NEW_DMART -eq 1 ]
  then
    near call $DMART_CTX_ID new '{"ft_contract": "'$NFT_CTX_ID'"}' --accountId $OWNER_ACCOUNT_ID
  else
    near call $DMART_CTX_ID set_ft_contract '{"ft_contract": "'$NFT_CTX_ID'"}' --accountId $DMART_CTX_ID
  fi
else
  #############################
  # Set up dMart      ###   ###
  #############################
  if [ $NEW_DMART -eq 1 ]
  then
    near call $NFT_CTX_ID set_role '{"account_id": "'$DMART_CTX_ID'"}' --accountId $OWNER_ACCOUNT_ID
    near call $DMART_CTX_ID new '{"ft_contract": "'$NFT_CTX_ID'"}' --accountId $OWNER_ACCOUNT_ID
  fi
fi

printf %b "\033[$attrib;${color}m|| --------------------------------------------\033[m\n"
Highlight_print_line "DMart contract id: $DMART_CTX_ID"
Highlight_print_line "NFT contract id: $NFT_CTX_ID"
Highlight_print_line "Owner account id: $OWNER_ACCOUNT_ID"
printf %b "\033[$attrib;${color}m|| --------------------------------------------\033[m\n"
