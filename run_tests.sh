BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

for i in {1..12}; 
do 
    crate_name="day-$i"
    if [ -d "$crate_name" ]; then 
        echo -e "${BLUE}Day $i RUNNING${NC}"
        if (cd "$crate_name" && cargo test --quiet); then
            echo -e "${GREEN}Day $i PASSED ${NC}\n\n"
        else
            echo -e "${RED}Day $i FAILED ${NC}\n\n"
        fi  
    fi
done
