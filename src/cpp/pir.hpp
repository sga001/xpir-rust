#ifndef PUNG_PIR_H
#define PUNG_PIR_H

#include <iostream>
#include "libpir.hpp"
#include "pir/dbhandlers/DBArrayProcessor.hpp"
#include <math.h>

using namespace std;

class DefaultPIRParams {
  private:
    PIRParameters params; 

  public:
    DefaultPIRParams(uint64_t num_db_entries) 
    {
      params.alpha = 1; 
      params.d = 1; 
      params.n[0] = num_db_entries; 
      params.crypto_params = "LWE:80:2048:60"; 
    }
    
    // This procedure is adapted from https://github.com/XPIR-team/XPIR/blob/master/apps/optim/PIROptimizer.cpp#L338
    void getDimSize(unsigned int n, unsigned int alpha, unsigned int d, unsigned int *dn)
    {
        unsigned int prod = 1, j = 0;
        
        // Elements in the database after the aggregation
        unsigned int new_n = ceil(static_cast<double>(n)/static_cast<double>(alpha)); //PAtch tres sale  reprendre
        
        // Dimension sizes. Product must be > n/alpha
        unsigned int factors[d];
        
        // Lower bound on the dimensions sizes needed. Correct only if n/alpha is a d-power.
        for (unsigned int i = 0 ; i < d ; i++) factors[i] = floor(pow(new_n,1./d));
        
        // If n/alpha is not a d-power
        if (static_cast<double>(factors[0]) != pow(new_n, static_cast<double>(1.0 / static_cast<double>(d))) )
        {
            // Increment each factor by one until the product of the dimension sizes is above n/alpha
            while (prod < new_n && j < d)
            {
                prod = 1;
                factors[j++]++;
                for (unsigned int i = 0 ; i < d ; i++)
                    prod *= factors[i];
            }
        }
        
        // Copy the result to the output
        memcpy(dn, factors, sizeof(factors));
    } 
    
    DefaultPIRParams(uint64_t num_db_entries, uint64_t alpha, uint64_t d) 
    {
      params.alpha = alpha;
      params.d = d; 

      getDimSize(num_db_entries, alpha, d, params.n);
//      cout<<"Dimensions: "<<params.n[0]<<"\t"<<params.n[1]<<"\t"<<params.n[2]<<endl; 
      params.crypto_params = "LWE:80:2048:60"; 
    }

    PIRParameters getParams()
    {
      return params;
    }
};

class XPIRServer {

  private:
    HomomorphicCrypto *crypto;
    imported_database *imported_db;
    DBArrayProcessor *db;
    PIRParameters params;

  public:  
    XPIRServer(uint64_t, char*, uint64_t, PIRParameters);
    ~XPIRServer();
    char* processQuery(char*, uint64_t len, uint64_t len_element, uint64_t *rlen, uint64_t *rlen_element);
};

//  PIR client-related classes and methods
//
//
class PIRClient 
{
  private:
    HomomorphicCrypto *crypto;
    uint64_t maxFileBytesize;
    PIRParameters params;
    uint64_t lastChosenIdx;

  public:  
    PIRClient(PIRParameters, uint64_t);
    ~PIRClient();
    void updateDBParams(PIRParameters p, uint64_t);
    uint64_t generateQuery(uint64_t, vector<char*>*);
    char* processReply(char* r, uint64_t len, uint64_t len_element, uint64_t *rlen);
};


uint64_t client_generate_query_internal(PIRClient *pir, uint64_t chosen_idx, vector<char*>* q);

extern "C" {

  void* cpp_server_setup(uint64_t len_db_total_bytes, char *db, uint64_t num_db_entries, uint64_t alpha, uint64_t d); 
  void* cpp_client_setup(uint64_t len_db_total_bytes, uint64_t num_db_entries, uint64_t alpha, uint64_t d);

  void cpp_server_free(void* pir);
  void cpp_client_free(void* pir);
  void cpp_buffer_free(char* buf);

  char* cpp_client_generate_query(void* pir, uint64_t chosen_idx, uint64_t* rlen_query_total_bytes, uint64_t* rnum_query_slots);
  char* cpp_server_process_query(void* pir, char* q, uint64_t len_query_total_bytes, uint64_t num_query_slots, uint64_t* rlen_response_total_bytes, uint64_t* rnum_response_slots);
  char* cpp_client_process_reply(void* pir, char* r, uint64_t len_response_total_bytes, uint64_t num_response_slots, uint64_t* rlen_answer_total_bytes);
  void cpp_client_update_db_params(void* pir, uint64_t len_db_total_bytes, uint64_t num_db_entries, uint64_t alpha, uint64_t d);
}
#endif
